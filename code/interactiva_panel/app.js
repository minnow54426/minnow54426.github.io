class PolynomialPlotter {
    constructor() {
        this.polynomials = [];
        this.maxPolynomials = 5;
        this.colors = ['#3498db', '#e74c3c', '#2ecc71', '#f39c12', '#9b59b6'];
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
        document.getElementById('add-polynomial').addEventListener('click', () => this.addPolynomial());

        // Range controls
        ['x-min', 'x-max', 'y-min', 'y-max'].forEach(id => {
            document.getElementById(id).addEventListener('input', (e) => this.updateRange(id, e.target.value));
        });
    }

    addPolynomial() {
        if (this.polynomials.length >= this.maxPolynomials) {
            this.showMessage(`Maximum ${this.maxPolynomials} polynomials allowed`, 'error');
            return;
        }

        const polynomial = {
            id: Date.now(),
            color: this.colors[this.polynomials.length],
            coefficients: [0, 0, 0, 1] // [c, bx, ax², dx³] - default to x³
        };

        this.polynomials.push(polynomial);
        this.createPolynomialControls(polynomial);
        this.updatePlot();
    }

    createPolynomialControls(polynomial) {
        const container = document.getElementById('polynomials-list');
        const polynomialDiv = document.createElement('div');
        polynomialDiv.className = 'polynomial-item';
        polynomialDiv.id = `polynomial-${polynomial.id}`;

        polynomialDiv.innerHTML = `
            <div class="polynomial-header">
                <span class="polynomial-color" style="background-color: ${polynomial.color}"></span>
                <span class="polynomial-equation">${this.getEquation(polynomial.coefficients)}</span>
                <button class="remove-polynomial" onclick="plotter.removePolynomial(${polynomial.id})">Remove</button>
            </div>
            <div class="coefficient-controls">
                ${this.createCoefficientControls(polynomial)}
            </div>
        `;

        container.appendChild(polynomialDiv);

        // Add event listeners for sliders
        polynomialDiv.querySelectorAll('.coefficient-slider').forEach((slider, index) => {
            slider.addEventListener('input', (e) => {
                this.updateCoefficient(polynomial.id, index, parseFloat(e.target.value));
            });
        });
    }

    createCoefficientControls(polynomial) {
        const labels = ['c', 'bx', 'ax²', 'dx³'];
        return polynomial.coefficients.map((value, index) => `
            <div class="coefficient-item">
                <label class="coefficient-label">${labels[index]}:</label>
                <input type="range" class="coefficient-slider"
                       min="-5" max="5" step="0.1" value="${value}">
                <span class="coefficient-value">${value.toFixed(1)}</span>
            </div>
        `).join('');
    }

    updateCoefficient(polynomialId, coefficientIndex, value) {
        const polynomial = this.polynomials.find(p => p.id === polynomialId);
        if (polynomial) {
            polynomial.coefficients[coefficientIndex] = value;

            // Update display
            const polynomialDiv = document.getElementById(`polynomial-${polynomialId}`);
            polynomialDiv.querySelector('.polynomial-equation').textContent = this.getEquation(polynomial.coefficients);
            polynomialDiv.querySelectorAll('.coefficient-value')[coefficientIndex].textContent = value.toFixed(1);

            this.updatePlot();
        }
    }

    removePolynomial(polynomialId) {
        this.polynomials = this.polynomials.filter(p => p.id !== polynomialId);
        document.getElementById(`polynomial-${polynomialId}`).remove();
        this.updatePlot();
    }

    updateRange(rangeId, value) {
        const numValue = parseFloat(value);

        if (isNaN(numValue)) {
            this.showMessage('Invalid range value', 'error');
            return;
        }

        // Validate range limits
        if (Math.abs(numValue) > 1000) {
            this.showMessage('Range values must be between -1000 and 1000', 'error');
            document.getElementById(rangeId).value = this.plotRange[this.getRangeKey(rangeId)];
            return;
        }

        const key = this.getRangeKey(rangeId);
        this.plotRange[key] = numValue;

        // Validate that min <= max
        if (this.plotRange.xMin >= this.plotRange.xMax) {
            this.showMessage('X minimum must be less than X maximum', 'error');
            document.getElementById(rangeId).value = this.plotRange[key];
            return;
        }

        if (this.plotRange.yMin >= this.plotRange.yMax) {
            this.showMessage('Y minimum must be less than Y maximum', 'error');
            document.getElementById(rangeId).value = this.plotRange[key];
            return;
        }

        this.updatePlot();
    }

    getRangeKey(rangeId) {
        const mapping = {
            'x-min': 'xMin',
            'x-max': 'xMax',
            'y-min': 'yMin',
            'y-max': 'yMax'
        };
        return mapping[rangeId];
    }

    getEquation(coefficients) {
        const [c, bx, ax, dx] = coefficients;
        let equation = 'y = ';

        if (dx !== 0) equation += `${dx.toFixed(1)}x³ `;
        if (ax !== 0) equation += `${ax >= 0 && dx !== 0 ? '+' : ''}${ax.toFixed(1)}x² `;
        if (bx !== 0) equation += `${bx >= 0 && (dx !== 0 || ax !== 0) ? '+' : ''}${bx.toFixed(1)}x `;
        if (c !== 0) equation += `${c >= 0 && (dx !== 0 || ax !== 0 || bx !== 0) ? '+' : ''}${c.toFixed(1)}`;

        if (equation === 'y = ') equation = 'y = 0';

        return equation;
    }

    evaluatePolynomial(coefficients, x) {
        const [c, bx, ax, dx] = coefficients;
        return c + bx * x + ax * x * x + dx * x * x * x;
    }

    findRoots(coefficients) {
        const [c, bx, ax, dx] = coefficients;
        const roots = [];

        // Handle different polynomial degrees
        if (dx !== 0) {
            // Cubic polynomial - simplified root finding
            // This is a basic implementation, could be improved with more sophisticated methods
            const discriminant = this.calculateCubicDiscriminant(c, bx, ax, dx);
            if (discriminant >= 0) {
                // Try to find at least one real root
                const root = this.findCubicRoot(coefficients);
                if (root !== null) roots.push(root);
            }
        } else if (ax !== 0) {
            // Quadratic polynomial
            const discriminant = bx * bx - 4 * ax * c;
            if (discriminant >= 0) {
                const sqrtDisc = Math.sqrt(discriminant);
                roots.push((-bx + sqrtDisc) / (2 * ax));
                if (discriminant > 0) {
                    roots.push((-bx - sqrtDisc) / (2 * ax));
                }
            }
        } else if (bx !== 0) {
            // Linear polynomial
            roots.push(-c / bx);
        }

        return roots.filter(root => !isNaN(root) && isFinite(root));
    }

    calculateCubicDiscriminant(c, bx, ax, dx) {
        // Simplified discriminant calculation
        const p = bx / dx;
        const q = ax / dx;
        const r = c / dx;
        return Math.pow(q, 2) - 4 * p * r;
    }

    findCubicRoot(coefficients) {
        // Newton-Raphson method for finding one root
        let x = 0;
        const [c, bx, ax, dx] = coefficients;
        const maxIterations = 100;
        const tolerance = 0.0001;

        for (let i = 0; i < maxIterations; i++) {
            const fx = this.evaluatePolynomial(coefficients, x);
            const fpx = bx + 2 * ax * x + 3 * dx * x * x;

            if (Math.abs(fpx) < tolerance) break;

            const newX = x - fx / fpx;
            if (Math.abs(newX - x) < tolerance) {
                return newX;
            }
            x = newX;
        }

        return null;
    }

    findExtrema(coefficients) {
        const [c, bx, ax, dx] = coefficients;
        const extrema = [];

        if (dx !== 0 || ax !== 0) {
            // Find critical points by setting derivative to zero
            // f'(x) = bx + 2ax + 3dx²
            const a = 3 * dx;
            const b = 2 * ax;
            const c_ = bx;

            if (a !== 0) {
                // Quadratic derivative (cubic original)
                const discriminant = b * b - 4 * a * c_;
                if (discriminant >= 0) {
                    const sqrtDisc = Math.sqrt(discriminant);
                    const x1 = (-b + sqrtDisc) / (2 * a);
                    const x2 = (-b - sqrtDisc) / (2 * a);

                    if (!isNaN(x1) && isFinite(x1)) {
                        const y1 = this.evaluatePolynomial(coefficients, x1);
                        const secondDerivative = 2 * b + 6 * a * x1;
                        extrema.push({ x: x1, y: y1, type: secondDerivative > 0 ? 'min' : 'max' });
                    }

                    if (discriminant > 0 && !isNaN(x2) && isFinite(x2)) {
                        const y2 = this.evaluatePolynomial(coefficients, x2);
                        const secondDerivative = 2 * b + 6 * a * x2;
                        extrema.push({ x: x2, y: y2, type: secondDerivative > 0 ? 'min' : 'max' });
                    }
                }
            } else if (b !== 0) {
                // Linear derivative (quadratic original)
                const x = -c_ / b;
                const y = this.evaluatePolynomial(coefficients, x);
                const secondDerivative = 2 * b;
                extrema.push({ x, y, type: secondDerivative > 0 ? 'min' : 'max' });
            }
        }

        return extrema;
    }

    updatePlot() {
        const traces = [];

        this.polynomials.forEach(polynomial => {
            // Generate plot data
            const xValues = [];
            const yValues = [];
            const step = (this.plotRange.xMax - this.plotRange.xMin) / 200;

            for (let x = this.plotRange.xMin; x <= this.plotRange.xMax; x += step) {
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

            // Add roots
            const roots = this.findRoots(polynomial.coefficients);
            if (roots.length > 0) {
                traces.push({
                    x: roots,
                    y: roots.map(x => this.evaluatePolynomial(polynomial.coefficients, x)),
                    type: 'scatter',
                    mode: 'markers',
                    name: `Roots ${this.polynomials.indexOf(polynomial) + 1}`,
                    marker: {
                        color: polynomial.color,
                        symbol: 'x',
                        size: 10,
                        line: {
                            color: 'white',
                            width: 2
                        }
                    },
                    showlegend: false
                });
            }

            // Add extrema
            const extrema = this.findExtrema(polynomial.coefficients);
            if (extrema.length > 0) {
                extrema.forEach(extremum => {
                    traces.push({
                        x: [extremum.x],
                        y: [extremum.y],
                        type: 'scatter',
                        mode: 'markers',
                        name: `${extremum.type === 'min' ? 'Minimum' : 'Maximum'} ${this.polynomials.indexOf(polynomial) + 1}`,
                        marker: {
                            color: polynomial.color,
                            symbol: extremum.type === 'min' ? 'triangle-up' : 'triangle-down',
                            size: 12,
                            line: {
                                color: 'white',
                                width: 2
                            }
                        },
                        showlegend: false
                    });
                });
            }
        });

        const layout = {
            title: 'Interactive Polynomial Plotter',
            xaxis: {
                title: 'X',
                range: [this.plotRange.xMin, this.plotRange.xMax],
                zeroline: true,
                zerolinecolor: '#888',
                zerolinewidth: 1,
                gridcolor: '#f0f0f0'
            },
            yaxis: {
                title: 'Y',
                range: [this.plotRange.yMin, this.plotRange.yMax],
                zeroline: true,
                zerolinecolor: '#888',
                zerolinewidth: 1,
                gridcolor: '#f0f0f0'
            },
            plot_bgcolor: '#fafafa',
            paper_bgcolor: 'white',
            margin: {
                l: 50,
                r: 50,
                t: 50,
                b: 50
            },
            legend: {
                x: 0,
                y: 1,
                bgcolor: 'rgba(255,255,255,0.8)',
                bordercolor: '#ddd',
                borderwidth: 1
            }
        };

        const config = {
            responsive: true,
            displayModeBar: true,
            modeBarButtonsToRemove: ['pan2d', 'lasso2d', 'select2d'],
            displaylogo: false
        };

        Plotly.newPlot('plot', traces, layout, config);
    }

    showMessage(message, type = 'info') {
        // Remove existing messages
        const existingMessages = document.querySelectorAll('.error-message, .info-message');
        existingMessages.forEach(msg => msg.remove());

        const messageDiv = document.createElement('div');
        messageDiv.className = `${type}-message`;
        messageDiv.textContent = message;

        const controlsSection = document.querySelector('.controls-section');
        controlsSection.appendChild(messageDiv);

        // Auto-remove after 3 seconds
        setTimeout(() => {
            messageDiv.remove();
        }, 3000);
    }
}

// Initialize the plotter when DOM is loaded
document.addEventListener('DOMContentLoaded', () => {
    window.plotter = new PolynomialPlotter();
});