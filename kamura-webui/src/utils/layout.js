export let options =
    {
        'cose': {
            name: 'cose',

            // Called on `layoutready`
            ready: function () {
            },

            // Called on `layoutstop`
            stop: function () {
            },

            // Whether to animate while running the layout
            // true : Animate continuously as the layout is running
            // false : Just show the end result
            // 'end' : Animate with the end result, from the initial positions to the end positions
            animate: true,

            // Easing of the animation for animate:'end'
            animationEasing: undefined,

            // The duration of the animation for animate:'end'
            animationDuration: undefined,

            // A function that determines whether the node should be animated
            // All nodes animated by default on animate enabled
            // Non-animated nodes are positioned immediately when the layout starts
            animateFilter: function (node, i) {
                return true;
            },


            // The layout animates only after this many milliseconds for animate:true
            // (prevents flashing on fast runs)
            animationThreshold: 250,

            // Number of iterations between consecutive screen positions update
            refresh: 20,

            // Whether to fit the network view after when done
            fit: true,

            // Padding on fit
            padding: 30,

            // Constrain layout bounds; { x1, y1, x2, y2 } or { x1, y1, w, h }
            boundingBox: undefined,

            // Excludes the label when calculating node bounding boxes for the layout algorithm
            nodeDimensionsIncludeLabels: false,

            // Randomize the initial positions of the nodes (true) or use existing positions (false)
            randomize: false,

            // Extra spacing between components in non-compound graphs
            componentSpacing: 40,

            // Node repulsion (non overlapping) multiplier
            nodeRepulsion: function (node) {
                return 2048;
            },

            // Node repulsion (overlapping) multiplier
            nodeOverlap: 4,

            // Ideal edge (non nested) length
            idealEdgeLength: function (edge) {
                return 32;
            },

            // Divisor to compute edge forces
            edgeElasticity: function (edge) {
                return 32;
            },

            // Nesting factor (multiplier) to compute ideal edge length for nested edges
            nestingFactor: 1.2,

            // Gravity force (constant)
            gravity: 1,

            // Maximum number of iterations to perform
            numIter: 1000,

            // Initial temperature (maximum node displacement)
            initialTemp: 1000,

            // Cooling factor (how the temperature is reduced between consecutive iterations
            coolingFactor: 0.99,

            // Lower temperature threshold (below this point the layout will end)
            minTemp: 1.0
        },
        'preset': {
            name: 'preset',

            positions: undefined, // map of (node id) => (position obj); or function(node){ return somPos; }
            zoom: undefined, // the zoom level to set (prob want fit = false if set)
            pan: undefined, // the pan level to set (prob want fit = false if set)
            fit: true, // whether to fit to viewport
            padding: 5, // padding on fit
            spacingFactor: undefined, // Applies a multiplicative factor (>0) to expand or compress the overall area that the nodes take up
            animate: false, // whether to transition the node positions
            animationDuration: 500, // duration of animation in ms if enabled
            animationEasing: undefined, // easing of animation if enabled
            animateFilter: function (node, i) {
                return true;
            }, // a function that determines whether the node should be animated.  All nodes animated by default on animate enabled.  Non-animated nodes are positioned immediately when the layout starts
            ready: undefined, // callback on layoutready
            stop: undefined, // callback on layoutstop
            transform: function (node, position) {
                return position;
            } // transform a given node position. Useful for changing flow direction in discrete layouts
        }
    }

export let styles = {
    'default': [
        {
            selector: 'node',
            style: {
                'background-color': '#fff',
                'border-color': '#000',
                'border-width': '1px',
                'label': 'data(label)',
                'text-valign': 'center',
                'color': '#000',
                'width': '40px',
                'height': '40px',
            }
        },
        {
            selector: '.subgraph',
            style: {
                'background-color': '#ddd',
                'shape': 'rectangle',
                'padding': '10px',
                'border-color': '#333',
                'border-width': '2px'
            }
        },
        {
            selector: 'edge',
            style: {
                'line-color': '#111',
                'target-arrow-color': '#000',
                'target-arrow-shape': 'triangle',
                'curve-style': 'bezier'
            }
        }
    ]
}