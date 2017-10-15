"use strict";

define([ "knockout", "moment" ], function(ko, moment) {
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
});
