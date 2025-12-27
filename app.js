class PolynomialPlotter {
    constructor() {
        // Constants
        this.MAX_POLYNOMIALS = 5;
        this.PLOT_STEP = 0.1;
        this.COEFFICIENT_COUNT = 6;

        this.polynomials = [];
        this.colors = ['#FF6B6B', '#4ECDC4', '#45B7D1', '#96CEB4', '#FECA57'];
        this.plotRange = {
            xMin: -10,
            xMax: 10,
            yMin: -10,
            yMax: 10
        };
        this.init();
    }

    init() {
        this.setupEventListeners();
        this.updatePlot();
    }

    setupEventListeners() {
        const addButton = document.getElementById('add-polynomial');
        if (addButton) {
            addButton.addEventListener('click', () => {
                this.addPolynomial();
            });
        }

        ['x-min', 'x-max', 'y-min', 'y-max'].forEach(id => {
            const element = document.getElementById(id);
            if (element) {
                element.addEventListener('change', (e) => {
                    this.updatePlotRange();
                });
            }
        });
    }

    addPolynomial() {
        if (this.polynomials.length >= this.MAX_POLYNOMIALS) {
            alert(`Maximum ${this.MAX_POLYNOMIALS} polynomials allowed`);
            return;
        }

        const polynomial = {
            id: Date.now(),
            coefficients: new Array(this.COEFFICIENT_COUNT).fill(0), // a0 through a5
            color: this.colors[this.polynomials.length]
        };

        this.polynomials.push(polynomial);
        this.createPolynomialControls(polynomial);
        this.updatePlot();
    }

    removePolynomial(id) {
        this.polynomials = this.polynomials.filter(p => p.id !== id);
        const element = document.getElementById(`polynomial-${id}`);
        if (element) {
            element.remove();
        }
        this.updatePlot();
    }

    createPolynomialControls(polynomial) {
        const container = document.getElementById('polynomials-list');
        if (!container) {
            console.error('Polynomials list container not found');
            return;
        }

        const card = document.createElement('div');
        card.className = 'polynomial-card';
        card.id = `polynomial-${polynomial.id}`;

        card.innerHTML = `
            <div class="polynomial-header">
                <h4 style="color: ${polynomial.color}">Polynomial ${this.polynomials.length}</h4>
                <button class="remove-polynomial" data-polynomial-id="${polynomial.id}">Remove</button>
            </div>
            <div class="equation" id="equation-${polynomial.id}">0</div>
            ${this.createCoefficientSliders(polynomial)}
        `;

        container.appendChild(card);

        // Add event listener to remove button
        const removeButton = card.querySelector('.remove-polynomial');
        if (removeButton) {
            removeButton.addEventListener('click', (e) => {
                const polynomialId = parseInt(e.target.dataset.polynomialId);
                this.removePolynomial(polynomialId);
            });
        }

        // Add event listeners to sliders
        for (let i = 0; i < this.COEFFICIENT_COUNT; i++) {
            const slider = card.querySelector(`#coeff-${polynomial.id}-${i}`);
            if (slider) {
                slider.addEventListener('input', (e) => {
                    const value = parseFloat(e.target.value);
                    if (!isNaN(value)) {
                        polynomial.coefficients[i] = value;
                        this.updateEquation(polynomial);
                        this.updatePlot();
                    }
                });
            }
        }

        this.updateEquation(polynomial);
    }

    createCoefficientSliders(polynomial) {
        const terms = ['a₀ (constant)', 'a₁ (x)', 'a₂ (x²)', 'a₃ (x³)', 'a₄ (x⁴)', 'a₅ (x⁵)'];
        let html = '';

        for (let i = 0; i < this.COEFFICIENT_COUNT; i++) {
            html += `
                <div class="coefficient-slider">
                    <label>
                        ${terms[i]}
                        <span class="coefficient-value" id="value-${polynomial.id}-${i}">0.0</span>
                    </label>
                    <input type="range"
                           id="coeff-${polynomial.id}-${i}"
                           min="-10"
                           max="10"
                           step="0.1"
                           value="0">
                </div>
            `;
        }

        return html;
    }

    updateEquation(polynomial) {
        const equation = this.formatEquation(polynomial.coefficients);
        const equationElement = document.getElementById(`equation-${polynomial.id}`);
        if (equationElement) {
            equationElement.textContent = equation;
        }

        // Update coefficient value displays
        for (let i = 0; i < this.COEFFICIENT_COUNT; i++) {
            const valueElement = document.getElementById(`value-${polynomial.id}-${i}`);
            if (valueElement) {
                valueElement.textContent = polynomial.coefficients[i].toFixed(1);
            }
        }
    }

    formatEquation(coefficients) {
        const terms = [];
        const powers = ['x⁵', 'x⁴', 'x³', 'x²', 'x', ''];

        for (let i = 0; i < this.COEFFICIENT_COUNT; i++) {
            const coeff = coefficients[this.COEFFICIENT_COUNT - 1 - i];
            const power = powers[this.COEFFICIENT_COUNT - 1 - i];

            if (coeff !== 0) {
                let term = '';
                if (coeff < 0) {
                    term += '- ';
                } else if (terms.length > 0) {
                    term += '+ ';
                }

                const absCoeff = Math.abs(coeff);
                if (absCoeff !== 1 || power === '') {
                    term += absCoeff.toFixed(1);
                }

                term += power;
                terms.push(term);
            }
        }

        return terms.length > 0 ? terms.join(' ') : '0';
    }

    updatePlotRange() {
        const xMinElement = document.getElementById('x-min');
        const xMaxElement = document.getElementById('x-max');
        const yMinElement = document.getElementById('y-min');
        const yMaxElement = document.getElementById('y-max');

        // Validate and parse input values
        const xMin = xMinElement ? parseFloat(xMinElement.value) : this.plotRange.xMin;
        const xMax = xMaxElement ? parseFloat(xMaxElement.value) : this.plotRange.xMax;
        const yMin = yMinElement ? parseFloat(yMinElement.value) : this.plotRange.yMin;
        const yMax = yMaxElement ? parseFloat(yMaxElement.value) : this.plotRange.yMax;

        // Only update if all values are valid numbers
        if (!isNaN(xMin) && !isNaN(xMax) && !isNaN(yMin) && !isNaN(yMax)) {
            // Validate range logic
            if (xMin < xMax && yMin < yMax) {
                this.plotRange.xMin = xMin;
                this.plotRange.xMax = xMax;
                this.plotRange.yMin = yMin;
                this.plotRange.yMax = yMax;
                this.updatePlot();
            } else {
                console.error('Invalid range: minimum values must be less than maximum values');
                // Reset input fields to current valid values
                if (xMinElement) xMinElement.value = this.plotRange.xMin;
                if (xMaxElement) xMaxElement.value = this.plotRange.xMax;
                if (yMinElement) yMinElement.value = this.plotRange.yMin;
                if (yMaxElement) yMaxElement.value = this.plotRange.yMax;
            }
        } else {
            console.error('Invalid input: please enter valid numbers for all range values');
        }
    }

    evaluatePolynomial(coefficients, x) {
        // Horner's method for efficiency
        let result = 0;
        for (let i = coefficients.length - 1; i >= 0; i--) {
            result = result * x + coefficients[i];
        }
        return result;
    }

    getRelativeTolerance(coefficients) {
        // Calculate tolerance based on coefficient scales
        const maxCoeff = Math.max(...coefficients.map(Math.abs));
        const minCoeff = Math.min(...coefficients.map(Math.abs).filter(c => c > 0));
        return maxCoeff > 0 ? Math.max(1e-10, maxCoeff * 1e-12) : 1e-10;
    }

    findRoots(coefficients) {
        const roots = [];
        const tolerance = this.getRelativeTolerance(coefficients);

        // For linear polynomials
        if (this.getDegree(coefficients) === 1) {
            // Check if coefficient of x (coefficients[1]) is zero
            if (Math.abs(coefficients[1]) < tolerance) {
                // This is actually a constant polynomial, no roots unless constant is zero
                if (Math.abs(coefficients[0]) < tolerance) {
                    // Infinite roots - represent as no specific points
                    return [];
                }
                return []; // Constant non-zero polynomial has no roots
            }
            const root = -coefficients[0] / coefficients[1];
            if (this.isInPlotRange(root, 0)) {
                roots.push({x: root, y: 0});
            }
        }
        // For quadratic polynomials
        else if (this.getDegree(coefficients) === 2) {
            const a = coefficients[2];
            const b = coefficients[1];
            const c = coefficients[0];

            // Handle near-zero coefficient 'a' - treat as linear
            if (Math.abs(a) < tolerance) {
                if (Math.abs(b) < tolerance) {
                    // Both a and b are near zero, treat as constant
                    if (Math.abs(c) < tolerance) {
                        return []; // Infinite roots case
                    }
                    return []; // Constant non-zero polynomial
                }
                // Linear case: bx + c = 0
                const root = -c / b;
                if (this.isInPlotRange(root, 0)) {
                    roots.push({x: root, y: 0});
                }
            } else {
                // Standard quadratic formula
                const discriminant = b * b - 4 * a * c;
                if (discriminant >= -tolerance) {
                    // Handle near-zero discriminant
                    const adjustedDiscriminant = Math.max(0, discriminant);
                    const sqrtDisc = Math.sqrt(adjustedDiscriminant);
                    const denominator = 2 * a;

                    const root1 = (-b + sqrtDisc) / denominator;
                    const root2 = (-b - sqrtDisc) / denominator;

                    if (this.isInPlotRange(root1, 0)) {
                        roots.push({x: root1, y: 0});
                    }
                    // Check if second root is distinct and in range
                    if (this.isInPlotRange(root2, 0) && Math.abs(root2 - root1) > tolerance) {
                        roots.push({x: root2, y: 0});
                    }
                }
            }
        }
        // For higher degrees, use numerical methods
        else {
            const numericalRoots = this.findNumericalRoots(coefficients);
            roots.push(...numericalRoots);
        }

        return roots;
    }

    findNumericalRoots(coefficients) {
        const roots = [];
        const step = 0.1;
        let prevY = this.evaluatePolynomial(coefficients, this.plotRange.xMin);

        for (let x = this.plotRange.xMin + step; x <= this.plotRange.xMax; x += step) {
            const y = this.evaluatePolynomial(coefficients, x);

            // Check for sign change (root between prevX and x)
            if (prevY === 0) {
                roots.push({x: x - step, y: 0});
            } else if (y === 0) {
                roots.push({x: x, y: 0});
            } else if (prevY * y < 0) {
                // Use bisection method for refinement
                const root = this.bisectionMethod(coefficients, x - step, x);
                if (root !== null) {
                    roots.push({x: root, y: 0});
                }
            }

            prevY = y;
        }

        return roots;
    }

    bisectionMethod(coefficients, a, b, tolerance = null) {
        // Validate input: ensure a < b
        if (a >= b) {
            // Swap if a > b
            [a, b] = [b, a];
        }

        // Use relative tolerance if not provided
        if (tolerance === null) {
            tolerance = this.getRelativeTolerance(coefficients);
        }

        let fa = this.evaluatePolynomial(coefficients, a);
        let fb = this.evaluatePolynomial(coefficients, b);

        // Check for sign change or zero values
        if (fa * fb > 0) {
            // No sign change, check if either endpoint is a root
            if (Math.abs(fa) < tolerance) return a;
            if (Math.abs(fb) < tolerance) return b;
            return null; // No root in interval
        }

        // Handle cases where one endpoint is exactly a root
        if (Math.abs(fa) < tolerance) return a;
        if (Math.abs(fb) < tolerance) return b;

        // Maximum iterations to prevent infinite loops
        const maxIterations = 100;
        let iteration = 0;

        while (Math.abs(b - a) > tolerance && iteration < maxIterations) {
            const c = (a + b) / 2;
            const fc = this.evaluatePolynomial(coefficients, c);

            if (Math.abs(fc) < tolerance) return c;

            if (fa * fc < 0) {
                b = c;
                fb = fc;
            } else {
                a = c;
                fa = fc;
            }
            iteration++;
        }

        // Return the midpoint if max iterations reached
        return (a + b) / 2;
    }

    findExtrema(coefficients) {
        const extrema = [];
        const degree = this.getDegree(coefficients);

        if (degree < 2) return extrema;

        // Calculate derivative coefficients
        const derivativeCoeffs = [];
        for (let i = 1; i < coefficients.length; i++) {
            derivativeCoeffs.push(i * coefficients[i]);
        }

        // Find critical points (roots of derivative)
        const criticalPoints = this.findRoots(derivativeCoeffs);

        // Evaluate second derivative to classify as max/min
        criticalPoints.forEach(point => {
            const secondDerivValue = this.evaluateSecondDerivative(coefficients, point.x);
            const yValue = this.evaluatePolynomial(coefficients, point.x);

            if (this.isInPlotRange(point.x, yValue)) {
                extrema.push({
                    x: point.x,
                    y: yValue,
                    type: secondDerivValue < 0 ? 'maximum' : 'minimum'
                });
            }
        });

        return extrema;
    }

    evaluateSecondDerivative(coefficients, x) {
        // Calculate second derivative coefficients
        const secondDerivCoeffs = [];
        for (let i = 2; i < coefficients.length; i++) {
            secondDerivCoeffs.push(i * (i - 1) * coefficients[i]);
        }

        return this.evaluatePolynomial(secondDerivCoeffs, x);
    }

    getDegree(coefficients) {
        const tolerance = this.getRelativeTolerance(coefficients);
        for (let i = coefficients.length - 1; i >= 0; i--) {
            if (Math.abs(coefficients[i]) > tolerance) {
                return i;
            }
        }
        return 0;
    }

    isInPlotRange(x, y) {
        return x >= this.plotRange.xMin && x <= this.plotRange.xMax &&
               y >= this.plotRange.yMin && y <= this.plotRange.yMax;
    }

    updatePlot() {
        const plotElement = document.getElementById('plot');
        if (!plotElement) {
            console.error('Plot element not found');
            return;
        }

        const traces = [];

        this.polynomials.forEach(polynomial => {
            const xValues = [];
            const yValues = [];

            // Generate points for the curve
            for (let x = this.plotRange.xMin; x <= this.plotRange.xMax; x += this.PLOT_STEP) {
                xValues.push(x);
                yValues.push(this.evaluatePolynomial(polynomial.coefficients, x));
            }

            traces.push({
                x: xValues,
                y: yValues,
                type: 'scatter',
                mode: 'lines',
                name: `Polynomial ${this.polynomials.indexOf(polynomial) + 1}`,
                line: {
                    color: polynomial.color,
                    width: 2
                }
            });

            // Add roots as scatter points
            const roots = this.findRoots(polynomial.coefficients);
            if (roots.length > 0) {
                traces.push({
                    x: roots.map(r => r.x),
                    y: roots.map(r => r.y),
                    type: 'scatter',
                    mode: 'markers',
                    name: `Roots ${this.polynomials.indexOf(polynomial) + 1}`,
                    marker: {
                        color: polynomial.color,
                        symbol: 'x',
                        size: 10,
                        line: { width: 2 }
                    }
                });
            }

            // Add extrema as scatter points
            const extrema = this.findExtrema(polynomial.coefficients);
            if (extrema.length > 0) {
                extrema.forEach(point => {
                    traces.push({
                        x: [point.x],
                        y: [point.y],
                        type: 'scatter',
                        mode: 'markers',
                        name: `${point.type} ${this.polynomials.indexOf(polynomial) + 1}`,
                        marker: {
                            color: polynomial.color,
                            symbol: point.type === 'maximum' ? 'triangle-down' : 'triangle-up',
                            size: 8
                        }
                    });
                });
            }
        });

        const layout = {
            title: 'Interactive Polynomial Plotter',
            xaxis: {
                title: 'x',
                range: [this.plotRange.xMin, this.plotRange.xMax],
                gridcolor: '#e0e0e0',
                zerolinecolor: '#666',
                zerolinewidth: 2
            },
            yaxis: {
                title: 'y',
                range: [this.plotRange.yMin, this.plotRange.yMax],
                gridcolor: '#e0e0e0',
                zerolinecolor: '#666',
                zerolinewidth: 2
            },
            plot_bgcolor: '#f8f9fa',
            paper_bgcolor: 'white',
            showlegend: true,
            hovermode: 'x unified'
        };

        try {
            Plotly.newPlot('plot', traces, layout, {responsive: true});
        } catch (error) {
            console.error('Error updating plot:', error);
        }
    }
}

// Initialize the application
let plotter;
document.addEventListener('DOMContentLoaded', () => {
    plotter = new PolynomialPlotter();
});