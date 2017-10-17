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

			if (data.length > 1) {
				const options = valueUnwrapped.options();
				const g = new chart(element, data, options);

				valueUnwrapped._g = g;
			}
		}, update: function(element, valueAccessor, allBindings) {
			const value = valueAccessor();
			const valueUnwrapped = ko.unwrap(value);
			const data = valueUnwrapped.data();

			if (data.length > 1) {
				const g = valueUnwrapped._g;
				const options = valueUnwrapped.options();
				const updateOptions = {};

				Object.assign(updateOptions, options);

				if (g) {
					Object.assign(updateOptions, { "file": data });

					g.updateOptions( updateOptions );
				} else {
					valueUnwrapped._g = new chart(element, data, options);
				}
			}
		}
	};
});
