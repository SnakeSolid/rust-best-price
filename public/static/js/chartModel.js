"use strict";

define([ "knockout", "moment" ], function(ko, moment) {
	const byIntegerValue = function(left, right) {
		if (left < right) {
			return -1;
		} else if (left > right) {
			return 1;
		} else {
			return 0;
		}
	};

	return function() {
		const self = this;

		this.data = ko.observableArray([]);
		this.options = ko.observable({
			width: 1097,
			height: 320,
			drawGapEdgePoints: true,
			highlightCircleSize: 2.5,
			labelsSeparateLines: true,
			panEdgeFraction: 0.25,
			showLabelsOnHighlight: true,
			stepPlot: true,
			strokeBorderWidth: 2.5,
			labels: [ "Update time", "-" ],
		});

		this.isVisible = ko.pureComputed(function() {
			return this.data().length > 1;
		}, this);

		this.hide = function() {
			this.data([]);
		};

		this.setData = function(data) {
			const labels = [ "Update time" ];
			const iterations = [];
			const data_map = {};
			const data_sample = data.map(function() {
				return null;
			});

			data.forEach(function (item, index) {
				labels.push(item.product);

				item.prices.forEach(function (price) {
					const iteration = price.iteration;

					if (!data_map.hasOwnProperty(iteration)) {
						iterations.push(iteration);

						const values = data_sample.slice([]);

						values[index] = price.price;
						data_map[iteration] = {
							count: 1,
							timestamp_sum: price.timestamp,
							values: values,
						};
					} else {
						const data_entry = data_map[iteration];

						data_entry.count += 1;
						data_entry.timestamp_sum += price.timestamp;
						data_entry.values[index] = price.price;
					}
				});
			});

			iterations.sort(byIntegerValue);

			const data_points = iterations.map(function(iteration) {
				const data_entry = data_map[iteration];
				const timestamp = data_entry.timestamp_sum / data_entry.count;
				const values = [ moment.unix(timestamp).toDate() ].concat(data_entry.values);

				return values;
			});

			self.options().labels = labels;
			self.data(data_points);
		};
	};
});
