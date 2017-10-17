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

		this.hasMessages = ko.pureComputed(function() {
			return this.messages().length > 0;
		}, this);

		this.isTableVisible = ko.pureComputed(function() {
			return this.products().length > 0;
		}, this);

		this.isChartVisible = ko.pureComputed(function() {
			return this.chart.isVisible();
		}, this);

		this.showChart = function(data, event) {
			reqwest({
				url: "/api/v1/price?category=" + data.category_id,
				method: "get",
				type: "json",
				contentType: "application/json"
			}).then(function (resp) {
				if (resp.ok) {
					self.chart.setData(resp.products);
				} else {
					self.messages.push(message.warn(resp.message, "Product price"));
				}
			}).fail(function(err) {
				self.messages.push(message.error("Failed to load price list", "Price list"));
			});
		};

		reqwest({
			url: "/api/v1/product",
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
