class PolynomialPlotter {
    constructor() {
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
        document.getElementById('add-polynomial').addEventListener('click', () => {
            this.addPolynomial();
        });

        ['x-min', 'x-max', 'y-min', 'y-max'].forEach(id => {
            document.getElementById(id).addEventListener('change', (e) => {
                this.updatePlotRange();
            });
        });
    }

    addPolynomial() {
        if (this.polynomials.length >= 5) {
            alert('Maximum 5 polynomials allowed');
            return;
        }

        const polynomial = {
            id: Date.now(),
            coefficients: [0, 0, 0, 0, 0, 0], // a₀ through a₅
            color: this.colors[this.polynomials.length]
        };

        this.polynomials.push(polynomial);
        this.createPolynomialControls(polynomial);
        this.updatePlot();
    }

    removePolynomial(id) {
        this.polynomials = this.polynomials.filter(p => p.id !== id);
        document.getElementById(`polynomial-${id}`).remove();
        this.updatePlot();
    }

    createPolynomialControls(polynomial) {
        const container = document.getElementById('polynomials-list');

        const card = document.createElement('div');
        card.className = 'polynomial-card';
        card.id = `polynomial-${polynomial.id}`;

        card.innerHTML = `
            <div class="polynomial-header">
                <h4 style="color: ${polynomial.color}">Polynomial ${this.polynomials.length}</h4>
                <button class="remove-polynomial" onclick="plotter.removePolynomial(${polynomial.id})">Remove</button>
            </div>
            <div class="equation" id="equation-${polynomial.id}">0</div>
            ${this.createCoefficientSliders(polynomial)}
        `;

        container.appendChild(card);

        // Add event listeners to sliders
        for (let i = 0; i < 6; i++) {
            const slider = card.querySelector(`#coeff-${polynomial.id}-${i}`);
            slider.addEventListener('input', (e) => {
                polynomial.coefficients[i] = parseFloat(e.target.value);
                this.updateEquation(polynomial);
                this.updatePlot();
            });
        }

        this.updateEquation(polynomial);
    }

    createCoefficientSliders(polynomial) {
        const terms = ['a₀ (constant)', 'a₁ (x)', 'a₂ (x²)', 'a₃ (x³)', 'a₄ (x⁴)', 'a₅ (x⁵)'];
        let html = '';

        for (let i = 0; i < 6; i++) {
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
        document.getElementById(`equation-${polynomial.id}`).textContent = equation;

        // Update coefficient value displays
        for (let i = 0; i < 6; i++) {
            const valueElement = document.getElementById(`value-${polynomial.id}-${i}`);
            if (valueElement) {
                valueElement.textContent = polynomial.coefficients[i].toFixed(1);
            }
        }
    }

    formatEquation(coefficients) {
        const terms = [];
        const powers = ['x⁵', 'x⁴', 'x³', 'x²', 'x', ''];

        for (let i = 0; i < 6; i++) {
            const coeff = coefficients[5 - i];
            const power = powers[5 - i];

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
        this.plotRange.xMin = parseFloat(document.getElementById('x-min').value);
        this.plotRange.xMax = parseFloat(document.getElementById('x-max').value);
        this.plotRange.yMin = parseFloat(document.getElementById('y-min').value);
        this.plotRange.yMax = parseFloat(document.getElementById('y-max').value);
        this.updatePlot();
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
        const traces = [];

        this.polynomials.forEach(polynomial => {
            const xValues = [];
            const yValues = [];

            // Generate points for the curve
            for (let x = this.plotRange.xMin; x <= this.plotRange.xMax; x += 0.1) {
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

        Plotly.newPlot('plot', traces, layout, {responsive: true});
    }
}

// Initialize the application
let plotter;
document.addEventListener('DOMContentLoaded', () => {
    plotter = new PolynomialPlotter();
});