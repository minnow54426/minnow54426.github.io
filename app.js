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
        });

        const layout = {
            title: 'Interactive Polynomial Plotter',
            xaxis: {
                title: 'x',
                range: [this.plotRange.xMin, this.plotRange.xMax],
                gridcolor: '#e0e0e0',
                zerolinecolor: '#666'
            },
            yaxis: {
                title: 'y',
                range: [this.plotRange.yMin, this.plotRange.yMax],
                gridcolor: '#e0e0e0',
                zerolinecolor: '#666'
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