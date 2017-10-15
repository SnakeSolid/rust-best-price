"use strict";

define([ "knockout", "reqwest", "messageModel", "chartModel" ], function(ko, reqwest, message, chart) {
	const byCategory = function(a, b) {
		if (a.category < b.category) {
			return -1;
		} else if (a.category > b.category) {
			return 1;
		} else {
			return 0;
		}
	};

	return function() {
		const self = this;

		this.messages = ko.observableArray([]);
		this.products = ko.observableArray([]);
		this.chart = new chart();

		this.isTableVisible = ko.pureComputed(function() {
			return this.products().length > 0;
		}, this);

		this.hasMessages = ko.pureComputed(function() {
			return this.messages().length > 0;
		}, this);

		reqwest({
			url: "/api/v1/price",
			method: "get",
			type: "json",
			contentType: "application/json"
		}).then(function (resp) {
			if (resp.ok) {
				self.products(resp.products.sort(byCategory));
			} else {
				self.messages.push(message.warn(resp.message, "Product price"));
			}
		}).fail(function (err) {
			self.messages.push(message.error("Failed to load products", "Product price"));
		});
	};
});
