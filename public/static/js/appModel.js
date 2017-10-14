"use strict";

define([ "knockout", "reqwest", "messageModel" ], function(ko, reqwest, message) {
	return function() {
		const self = this;

		this.messages = ko.observableArray([]);
		this.products = ko.observableArray([]);

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
			self.products(resp);
		}).fail(function (err) {
			self.messages.push(message.error("Failed to load products", "Product price"));
		});
	};
});
