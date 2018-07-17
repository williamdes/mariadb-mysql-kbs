
window.projectVersion = 'master';

(function(root) {

    var bhIndex = null;
    var rootPath = '';
    var treeHtml = '        <ul>                <li data-name="namespace:Williamdes" class="opened">                    <div style="padding-left:0px" class="hd">                        <span class="glyphicon glyphicon-play"></span><a href="Williamdes.html">Williamdes</a>                    </div>                    <div class="bd">                                <ul>                <li data-name="namespace:Williamdes_MariaDBMySQLKBS" class="opened">                    <div style="padding-left:18px" class="hd">                        <span class="glyphicon glyphicon-play"></span><a href="Williamdes/MariaDBMySQLKBS.html">MariaDBMySQLKBS</a>                    </div>                    <div class="bd">                                <ul>                <li data-name="class:Williamdes_MariaDBMySQLKBS_KBDocumentation" >                    <div style="padding-left:44px" class="hd leaf">                        <a href="Williamdes/MariaDBMySQLKBS/KBDocumentation.html">KBDocumentation</a>                    </div>                </li>                            <li data-name="class:Williamdes_MariaDBMySQLKBS_KBEntry" >                    <div style="padding-left:44px" class="hd leaf">                        <a href="Williamdes/MariaDBMySQLKBS/KBEntry.html">KBEntry</a>                    </div>                </li>                            <li data-name="class:Williamdes_MariaDBMySQLKBS_KBException" >                    <div style="padding-left:44px" class="hd leaf">                        <a href="Williamdes/MariaDBMySQLKBS/KBException.html">KBException</a>                    </div>                </li>                            <li data-name="class:Williamdes_MariaDBMySQLKBS_Search" >                    <div style="padding-left:44px" class="hd leaf">                        <a href="Williamdes/MariaDBMySQLKBS/Search.html">Search</a>                    </div>                </li>                            <li data-name="class:Williamdes_MariaDBMySQLKBS_SlimData" >                    <div style="padding-left:44px" class="hd leaf">                        <a href="Williamdes/MariaDBMySQLKBS/SlimData.html">SlimData</a>                    </div>                </li>                </ul></div>                </li>                </ul></div>                </li>                </ul>';

    var searchTypeClasses = {
        'Namespace': 'label-default',
        'Class': 'label-info',
        'Interface': 'label-primary',
        'Trait': 'label-success',
        'Method': 'label-danger',
        '_': 'label-warning'
    };

    var searchIndex = [
                    
            {"type": "Namespace", "link": "Williamdes.html", "name": "Williamdes", "doc": "Namespace Williamdes"},{"type": "Namespace", "link": "Williamdes/MariaDBMySQLKBS.html", "name": "Williamdes\\MariaDBMySQLKBS", "doc": "Namespace Williamdes\\MariaDBMySQLKBS"},
            
            {"type": "Class", "fromName": "Williamdes\\MariaDBMySQLKBS", "fromLink": "Williamdes/MariaDBMySQLKBS.html", "link": "Williamdes/MariaDBMySQLKBS/KBDocumentation.html", "name": "Williamdes\\MariaDBMySQLKBS\\KBDocumentation", "doc": "&quot;&quot;"},
                                                        {"type": "Method", "fromName": "Williamdes\\MariaDBMySQLKBS\\KBDocumentation", "fromLink": "Williamdes/MariaDBMySQLKBS/KBDocumentation.html", "link": "Williamdes/MariaDBMySQLKBS/KBDocumentation.html#method___construct", "name": "Williamdes\\MariaDBMySQLKBS\\KBDocumentation::__construct", "doc": "&quot;Create a KBEntry object&quot;"},
                    {"type": "Method", "fromName": "Williamdes\\MariaDBMySQLKBS\\KBDocumentation", "fromLink": "Williamdes/MariaDBMySQLKBS/KBDocumentation.html", "link": "Williamdes/MariaDBMySQLKBS/KBDocumentation.html#method_getUrl", "name": "Williamdes\\MariaDBMySQLKBS\\KBDocumentation::getUrl", "doc": "&quot;Get the url&quot;"},
                    {"type": "Method", "fromName": "Williamdes\\MariaDBMySQLKBS\\KBDocumentation", "fromLink": "Williamdes/MariaDBMySQLKBS/KBDocumentation.html", "link": "Williamdes/MariaDBMySQLKBS/KBDocumentation.html#method_getAnchor", "name": "Williamdes\\MariaDBMySQLKBS\\KBDocumentation::getAnchor", "doc": "&quot;Get the anchor&quot;"},
                    {"type": "Method", "fromName": "Williamdes\\MariaDBMySQLKBS\\KBDocumentation", "fromLink": "Williamdes/MariaDBMySQLKBS/KBDocumentation.html", "link": "Williamdes/MariaDBMySQLKBS/KBDocumentation.html#method_jsonSerialize", "name": "Williamdes\\MariaDBMySQLKBS\\KBDocumentation::jsonSerialize", "doc": "&quot;Used for json_encode function\nThis can seem useless, do not remove it.&quot;"},
            
            {"type": "Class", "fromName": "Williamdes\\MariaDBMySQLKBS", "fromLink": "Williamdes/MariaDBMySQLKBS.html", "link": "Williamdes/MariaDBMySQLKBS/KBEntry.html", "name": "Williamdes\\MariaDBMySQLKBS\\KBEntry", "doc": "&quot;&quot;"},
                                                        {"type": "Method", "fromName": "Williamdes\\MariaDBMySQLKBS\\KBEntry", "fromLink": "Williamdes/MariaDBMySQLKBS/KBEntry.html", "link": "Williamdes/MariaDBMySQLKBS/KBEntry.html#method___construct", "name": "Williamdes\\MariaDBMySQLKBS\\KBEntry::__construct", "doc": "&quot;Create a KBEntry object&quot;"},
                    {"type": "Method", "fromName": "Williamdes\\MariaDBMySQLKBS\\KBEntry", "fromLink": "Williamdes/MariaDBMySQLKBS/KBEntry.html", "link": "Williamdes/MariaDBMySQLKBS/KBEntry.html#method_getName", "name": "Williamdes\\MariaDBMySQLKBS\\KBEntry::getName", "doc": "&quot;Get the variable name&quot;"},
                    {"type": "Method", "fromName": "Williamdes\\MariaDBMySQLKBS\\KBEntry", "fromLink": "Williamdes/MariaDBMySQLKBS/KBEntry.html", "link": "Williamdes/MariaDBMySQLKBS/KBEntry.html#method_isDynamic", "name": "Williamdes\\MariaDBMySQLKBS\\KBEntry::isDynamic", "doc": "&quot;Is the variable dynamic&quot;"},
                    {"type": "Method", "fromName": "Williamdes\\MariaDBMySQLKBS\\KBEntry", "fromLink": "Williamdes/MariaDBMySQLKBS/KBEntry.html", "link": "Williamdes/MariaDBMySQLKBS/KBEntry.html#method_getType", "name": "Williamdes\\MariaDBMySQLKBS\\KBEntry::getType", "doc": "&quot;Get the variable type&quot;"},
                    {"type": "Method", "fromName": "Williamdes\\MariaDBMySQLKBS\\KBEntry", "fromLink": "Williamdes/MariaDBMySQLKBS/KBEntry.html", "link": "Williamdes/MariaDBMySQLKBS/KBEntry.html#method_hasDocumentations", "name": "Williamdes\\MariaDBMySQLKBS\\KBEntry::hasDocumentations", "doc": "&quot;Variable has documentations&quot;"},
                    {"type": "Method", "fromName": "Williamdes\\MariaDBMySQLKBS\\KBEntry", "fromLink": "Williamdes/MariaDBMySQLKBS/KBEntry.html", "link": "Williamdes/MariaDBMySQLKBS/KBEntry.html#method_getDocumentations", "name": "Williamdes\\MariaDBMySQLKBS\\KBEntry::getDocumentations", "doc": "&quot;Get all documentations&quot;"},
                    {"type": "Method", "fromName": "Williamdes\\MariaDBMySQLKBS\\KBEntry", "fromLink": "Williamdes/MariaDBMySQLKBS/KBEntry.html", "link": "Williamdes/MariaDBMySQLKBS/KBEntry.html#method_addDocumentation", "name": "Williamdes\\MariaDBMySQLKBS\\KBEntry::addDocumentation", "doc": "&quot;Add cocumentation link&quot;"},
                    {"type": "Method", "fromName": "Williamdes\\MariaDBMySQLKBS\\KBEntry", "fromLink": "Williamdes/MariaDBMySQLKBS/KBEntry.html", "link": "Williamdes/MariaDBMySQLKBS/KBEntry.html#method_jsonSerialize", "name": "Williamdes\\MariaDBMySQLKBS\\KBEntry::jsonSerialize", "doc": "&quot;Used for json_encode function\nThis can seem useless, do not remove it.&quot;"},
            
            {"type": "Class", "fromName": "Williamdes\\MariaDBMySQLKBS", "fromLink": "Williamdes/MariaDBMySQLKBS.html", "link": "Williamdes/MariaDBMySQLKBS/KBException.html", "name": "Williamdes\\MariaDBMySQLKBS\\KBException", "doc": "&quot;KBException class&quot;"},
                    
            {"type": "Class", "fromName": "Williamdes\\MariaDBMySQLKBS", "fromLink": "Williamdes/MariaDBMySQLKBS.html", "link": "Williamdes/MariaDBMySQLKBS/Search.html", "name": "Williamdes\\MariaDBMySQLKBS\\Search", "doc": "&quot;&quot;"},
                                                        {"type": "Method", "fromName": "Williamdes\\MariaDBMySQLKBS\\Search", "fromLink": "Williamdes/MariaDBMySQLKBS/Search.html", "link": "Williamdes/MariaDBMySQLKBS/Search.html#method_loadData", "name": "Williamdes\\MariaDBMySQLKBS\\Search::loadData", "doc": "&quot;Load data from disk&quot;"},
                    {"type": "Method", "fromName": "Williamdes\\MariaDBMySQLKBS\\Search", "fromLink": "Williamdes/MariaDBMySQLKBS/Search.html", "link": "Williamdes/MariaDBMySQLKBS/Search.html#method_loadTestData", "name": "Williamdes\\MariaDBMySQLKBS\\Search::loadTestData", "doc": "&quot;Load test data&quot;"},
                    {"type": "Method", "fromName": "Williamdes\\MariaDBMySQLKBS\\Search", "fromLink": "Williamdes/MariaDBMySQLKBS/Search.html", "link": "Williamdes/MariaDBMySQLKBS/Search.html#method_getByName", "name": "Williamdes\\MariaDBMySQLKBS\\Search::getByName", "doc": "&quot;get the first link to doc available&quot;"},
                    {"type": "Method", "fromName": "Williamdes\\MariaDBMySQLKBS\\Search", "fromLink": "Williamdes/MariaDBMySQLKBS/Search.html", "link": "Williamdes/MariaDBMySQLKBS/Search.html#method_getVariable", "name": "Williamdes\\MariaDBMySQLKBS\\Search::getVariable", "doc": "&quot;Get a variable&quot;"},
                    {"type": "Method", "fromName": "Williamdes\\MariaDBMySQLKBS\\Search", "fromLink": "Williamdes/MariaDBMySQLKBS/Search.html", "link": "Williamdes/MariaDBMySQLKBS/Search.html#method_getVariableType", "name": "Williamdes\\MariaDBMySQLKBS\\Search::getVariableType", "doc": "&quot;get the type of the variable&quot;"},
                    {"type": "Method", "fromName": "Williamdes\\MariaDBMySQLKBS\\Search", "fromLink": "Williamdes/MariaDBMySQLKBS/Search.html", "link": "Williamdes/MariaDBMySQLKBS/Search.html#method_getStaticVariables", "name": "Williamdes\\MariaDBMySQLKBS\\Search::getStaticVariables", "doc": "&quot;Return the list of static variables&quot;"},
                    {"type": "Method", "fromName": "Williamdes\\MariaDBMySQLKBS\\Search", "fromLink": "Williamdes/MariaDBMySQLKBS/Search.html", "link": "Williamdes/MariaDBMySQLKBS/Search.html#method_getDynamicVariables", "name": "Williamdes\\MariaDBMySQLKBS\\Search::getDynamicVariables", "doc": "&quot;Return the list of dynamic variables&quot;"},
                    {"type": "Method", "fromName": "Williamdes\\MariaDBMySQLKBS\\Search", "fromLink": "Williamdes/MariaDBMySQLKBS/Search.html", "link": "Williamdes/MariaDBMySQLKBS/Search.html#method_getVariablesWithDynamic", "name": "Williamdes\\MariaDBMySQLKBS\\Search::getVariablesWithDynamic", "doc": "&quot;Return the list of variables having dynamic = $dynamic&quot;"},
            
            {"type": "Class", "fromName": "Williamdes\\MariaDBMySQLKBS", "fromLink": "Williamdes/MariaDBMySQLKBS.html", "link": "Williamdes/MariaDBMySQLKBS/SlimData.html", "name": "Williamdes\\MariaDBMySQLKBS\\SlimData", "doc": "&quot;&quot;"},
                                                        {"type": "Method", "fromName": "Williamdes\\MariaDBMySQLKBS\\SlimData", "fromLink": "Williamdes/MariaDBMySQLKBS/SlimData.html", "link": "Williamdes/MariaDBMySQLKBS/SlimData.html#method___construct", "name": "Williamdes\\MariaDBMySQLKBS\\SlimData::__construct", "doc": "&quot;Create a slimData object&quot;"},
                    {"type": "Method", "fromName": "Williamdes\\MariaDBMySQLKBS\\SlimData", "fromLink": "Williamdes/MariaDBMySQLKBS/SlimData.html", "link": "Williamdes/MariaDBMySQLKBS/SlimData.html#method_addVariable", "name": "Williamdes\\MariaDBMySQLKBS\\SlimData::addVariable", "doc": "&quot;Add a variable&quot;"},
                    {"type": "Method", "fromName": "Williamdes\\MariaDBMySQLKBS\\SlimData", "fromLink": "Williamdes/MariaDBMySQLKBS/SlimData.html", "link": "Williamdes/MariaDBMySQLKBS/SlimData.html#method_jsonSerialize", "name": "Williamdes\\MariaDBMySQLKBS\\SlimData::jsonSerialize", "doc": "&quot;Used for json_encode function\nThis can seem useless, do not remove it.&quot;"},
            
            
                                        // Fix trailing commas in the index
        {}
    ];

    /** Tokenizes strings by namespaces and functions */
    function tokenizer(term) {
        if (!term) {
            return [];
        }

        var tokens = [term];
        var meth = term.indexOf('::');

        // Split tokens into methods if "::" is found.
        if (meth > -1) {
            tokens.push(term.substr(meth + 2));
            term = term.substr(0, meth - 2);
        }

        // Split by namespace or fake namespace.
        if (term.indexOf('\\') > -1) {
            tokens = tokens.concat(term.split('\\'));
        } else if (term.indexOf('_') > 0) {
            tokens = tokens.concat(term.split('_'));
        }

        // Merge in splitting the string by case and return
        tokens = tokens.concat(term.match(/(([A-Z]?[^A-Z]*)|([a-z]?[^a-z]*))/g).slice(0,-1));

        return tokens;
    };

    root.Sami = {
        /**
         * Cleans the provided term. If no term is provided, then one is
         * grabbed from the query string "search" parameter.
         */
        cleanSearchTerm: function(term) {
            // Grab from the query string
            if (typeof term === 'undefined') {
                var name = 'search';
                var regex = new RegExp("[\\?&]" + name + "=([^&#]*)");
                var results = regex.exec(location.search);
                if (results === null) {
                    return null;
                }
                term = decodeURIComponent(results[1].replace(/\+/g, " "));
            }

            return term.replace(/<(?:.|\n)*?>/gm, '');
        },

        /** Searches through the index for a given term */
        search: function(term) {
            // Create a new search index if needed
            if (!bhIndex) {
                bhIndex = new Bloodhound({
                    limit: 500,
                    local: searchIndex,
                    datumTokenizer: function (d) {
                        return tokenizer(d.name);
                    },
                    queryTokenizer: Bloodhound.tokenizers.whitespace
                });
                bhIndex.initialize();
            }

            results = [];
            bhIndex.get(term, function(matches) {
                results = matches;
            });

            if (!rootPath) {
                return results;
            }

            // Fix the element links based on the current page depth.
            return $.map(results, function(ele) {
                if (ele.link.indexOf('..') > -1) {
                    return ele;
                }
                ele.link = rootPath + ele.link;
                if (ele.fromLink) {
                    ele.fromLink = rootPath + ele.fromLink;
                }
                return ele;
            });
        },

        /** Get a search class for a specific type */
        getSearchClass: function(type) {
            return searchTypeClasses[type] || searchTypeClasses['_'];
        },

        /** Add the left-nav tree to the site */
        injectApiTree: function(ele) {
            ele.html(treeHtml);
        }
    };

    $(function() {
        // Modify the HTML to work correctly based on the current depth
        rootPath = $('body').attr('data-root-path');
        treeHtml = treeHtml.replace(/href="/g, 'href="' + rootPath);
        Sami.injectApiTree($('#api-tree'));
    });

    return root.Sami;
})(window);

$(function() {

    // Enable the version switcher
    $('#version-switcher').change(function() {
        window.location = $(this).val()
    });

    
        // Toggle left-nav divs on click
        $('#api-tree .hd span').click(function() {
            $(this).parent().parent().toggleClass('opened');
        });

        // Expand the parent namespaces of the current page.
        var expected = $('body').attr('data-name');

        if (expected) {
            // Open the currently selected node and its parents.
            var container = $('#api-tree');
            var node = $('#api-tree li[data-name="' + expected + '"]');
            // Node might not be found when simulating namespaces
            if (node.length > 0) {
                node.addClass('active').addClass('opened');
                node.parents('li').addClass('opened');
                var scrollPos = node.offset().top - container.offset().top + container.scrollTop();
                // Position the item nearer to the top of the screen.
                scrollPos -= 200;
                container.scrollTop(scrollPos);
            }
        }

    
    
        var form = $('#search-form .typeahead');
        form.typeahead({
            hint: true,
            highlight: true,
            minLength: 1
        }, {
            name: 'search',
            displayKey: 'name',
            source: function (q, cb) {
                cb(Sami.search(q));
            }
        });

        // The selection is direct-linked when the user selects a suggestion.
        form.on('typeahead:selected', function(e, suggestion) {
            window.location = suggestion.link;
        });

        // The form is submitted when the user hits enter.
        form.keypress(function (e) {
            if (e.which == 13) {
                $('#search-form').submit();
                return true;
            }
        });

    
});


