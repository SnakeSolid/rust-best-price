"use strict";

define([ "knockout", "exports" ], function(ko, exports) {
	function InfoMessage(message, header = "") {
		this.isInfo = true;
		this.isWarning = false;
		this.isError = false;

		this.header = ko.observable(header);
		this.message = ko.observable(message);

		this.hasHeader = ko.pureComputed(function() {
			return this.header().length > 0;
		}, this);
	};

	function WarningMessage(message, header = "") {
		this.isInfo = false;
		this.isWarning = true;
		this.isError = false;

		this.header = ko.observable(header);
		this.message = ko.observable(message);

		this.hasHeader = ko.pureComputed(function() {
			return this.header().length > 0;
		}, this);
	};

	function ErrorMessage(message, header = "") {
		this.isInfo = false;
		this.isWarning = false;
		this.isError = true;

		this.header = ko.observable(header);
		this.message = ko.observable(message);

		this.hasHeader = ko.pureComputed(function() {
			return this.header().length > 0;
		}, this);
	};

	exports.info = function(message, header = "") {
		return new InfoMessage(message, header);
	};
	
	exports.warn = function(message, header = "") {
		return new WarningMessage(message, header);
	};
	
	exports.error = function(message, header = "") {
		return new ErrorMessage(message, header);
	};
});
