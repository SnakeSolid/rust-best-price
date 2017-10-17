"use strict";

define([ "knockout", "reqwest", "moment", "messageModel", "chartModel" ], function(ko, reqwest, moment, message, chart) {
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
		this.chartCategoryName = ko.observable("");
		this.lastUpdate = ko.observable(null);
		this.isProductsLoading = ko.observable(true);
		this.isChartLoading = ko.observable(true);

		this.lastUpdateText = ko.pureComputed(function() {
			const lastUpdate = this.lastUpdate();

			if (lastUpdate) {
				return lastUpdate.format("YYYY.DD.MM HH:mm:ss");
			} else {
				return "";
			}
		}, this);

		this.hasMessages = ko.pureComputed(function() {
			return this.messages().length > 0;
		}, this);

		this.isLastUpdateVisible = ko.pureComputed(function() {
			return this.lastUpdate() !== null;
		}, this);

		this.isRefreshVisible = ko.pureComputed(function() {
			return !this.isProductsLoading();
		}, this);

		this.isClearMessagesVisible = ko.pureComputed(function() {
			return this.messages().length > 0;
		}, this);

		this.isTableVisible = ko.pureComputed(function() {
			return this.products().length > 0;
		}, this);

		this.isChartVisible = ko.pureComputed(function() {
			return this.chart.isVisible();
		}, this);

		this.clearMessagesVisible = function() {
			self.messages([]);
		};

		this.showChart = function(data, event) {
			self.isChartLoading(true);
			self.chartCategoryName(data.category);

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

				self.isChartLoading(false);
			}).fail(function(err) {
				self.messages.push(message.error("Failed to load price list", "Price list"));

				self.isChartLoading(false);
			});
		};

		this.hideChart = function() {
			self.chart.hide();
		};

		this.loadProducts = function() {
			self.isProductsLoading(true);

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

				self.isProductsLoading(false);
				self.lastUpdate(moment());
			}).fail(function (err) {
				self.messages.push(message.error("Failed to load products", "Product price"));

				self.isProductsLoading(false);
			});
		};

		this.updateProducts = function() {
			if (!self.isProductsLoading()) {
				this.loadProducts();
			}
		};

		this.loadProducts();
	};
});
