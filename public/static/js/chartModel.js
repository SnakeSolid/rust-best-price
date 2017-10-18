"use strict";

define([ "knockout", "moment" ], function(ko, moment) {
	const byTimestamp = function(left, right) {
		if (left.timestamp < right.timestamp) {
			return -1;
		} else if (left.timestamp > right.timestamp) {
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
			const data_points = [];

			data.forEach(function (item, index) {
				labels.push(item.product);

				item.prices.forEach(function (price) {
					data_points.push({
						timestamp: price.timestamp,
						index: index,
						price: price.price,
					});
				});
			});

			data_points.sort(byTimestamp);

			const data_sample = data.map(function() {
				return null;
			});
			const data_values = data_points.map(function(point) {
				data_sample[point.index] = point.price;

				return [ moment.unix(point.timestamp).toDate() ].concat(data_sample);
			});

			self.options().labels = labels;
			self.data(data_values);
		};
	};
});
