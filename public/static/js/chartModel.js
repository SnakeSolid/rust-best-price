"use strict";

define([ "knockout" ], function(ko) {
	return function() {
		this.data = ko.observableArray([
			[ new Date(1508084250), 1 ],
			[ new Date(1508084253), 2 ],
			[ new Date(1508084255), 2 ],
			[ new Date(1508084257), 4 ],
			[ new Date(1508084263), 1 ],
		]);
		this.options = ko.observable({
			connectSeparatedPoints: true,
			drawGapEdgePoints: true,
		});
	};
});
