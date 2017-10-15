"use strict";

define([ "knockout", "moment", "chart" ], function(ko, moment, chart) {
	// Show unix timestamp in human readable format as time elapsed from now
	ko.bindingHandlers.asDuration = {
		update: function(element, valueAccessor, allBindings) {
			const value = valueAccessor();
			const valueUnwrapped = ko.unwrap(value);
			const unixTime = moment.unix(valueUnwrapped);

			element.innerText = unixTime.fromNow();
		}
	};

	// Show number as localized currency
	ko.bindingHandlers.asFixed = {
		update: function(element, valueAccessor, allBindings) {
			const value = valueAccessor();
			const valueUnwrapped = ko.unwrap(value);
			const text = valueUnwrapped.toLocaleString(undefined, {
				style: "currency",
				currency: "RUB",
				currencyDisplay: "code"
			});

			element.innerText = text;
		}
	};

	// Draw chart component from model
	ko.bindingHandlers.asChart = {
		init: function(element, valueAccessor, allBindings, _, bindingContext) {
			const value = valueAccessor();
			const valueUnwrapped = ko.unwrap(value);
			const data = valueUnwrapped.data();
			const options = valueUnwrapped.options();
			const g = new chart(element, data, options);

			valueUnwrapped._g = g;
		}, update: function(element, valueAccessor, allBindings) {
			const value = valueAccessor();
			const valueUnwrapped = ko.unwrap(value);
			const data = valueUnwrapped.data();
			const g = valueUnwrapped._g;

			g.updateOptions( { "file": data } );
		}
	};
});
