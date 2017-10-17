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
			connectSeparatedPoints: true,
			drawGapEdgePoints: true,
			stepPlot: true,
			labels: [ "Update time", "-" ],
			legend: "always",
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
				const values = data_sample.map(function (value, index) {
					if (index == point.index) {
						return point.price;
					} else {
						return value;
					}
				});

				return [ moment.unix(point.timestamp).toDate() ].concat(values);
			});

			self.options().labels = labels;
			self.data(data_values);
		};
	};
});
