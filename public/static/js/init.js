"use strict";

requirejs.config({
    baseUrl: "/static/js",
    paths: {
		reqwest: [ "https://cdnjs.cloudflare.com/ajax/libs/reqwest/2.0.5/reqwest.min", "lib/reqwest.min" ],
		semantic: [ "https://cdnjs.cloudflare.com/ajax/libs/semantic-ui/2.2.13/semantic.min", "lib/semantic.min" ],
		knockout: [ "https://cdnjs.cloudflare.com/ajax/libs/knockout/3.4.2/knockout-min", "lib/knockout-min" ],
		moment: [ "https://cdnjs.cloudflare.com/ajax/libs/moment.js/2.19.1/moment.min", "lib/moment.min" ],
    },
    shim: {
    	reqwest: {
			exports: 'reqwest',
    	}
    },
    waitSeconds: 15,
});

// Start the main app logic.
requirejs(["knockout", "appModel"],
	function(ko, appModel) {
	    ko.applyBindings(new appModel());
	}, function (err) {
	    console.log(err.requireType);

	    if (err.requireType === 'timeout') {
	        console.log('modules: ' + err.requireModules);
	    }

	    throw err;
	}
);
