"use strict";

define([ "knockout" ], function(ko) {
	return function() {
		this.data = ko.observableArray([]);
		this.options = ko.observable({
			connectSeparatedPoints: true,
			drawGapEdgePoints: true,
			labels: [ "Update time", "-" ],
			legend: "always",
			width: 1097,
		});

		this.isVisible = ko.pureComputed(function() {
			return this.date,length > 1;
		}, this);
	};
});
