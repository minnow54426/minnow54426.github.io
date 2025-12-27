# Interactive Polynomial Plotter Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Build an interactive web page where users can adjust polynomial coefficients with sliders and see real-time plot updates with support for multiple polynomial comparison.

**Architecture:** Single-page static web application using HTML5, CSS3, vanilla JavaScript with Plotly.js for plotting, featuring a split-screen layout with plot on left and controls on right.

**Tech Stack:** HTML5, CSS3, JavaScript (ES6+), Plotly.js library, CSS Grid for responsive layout

---

### Task 1: Create Basic HTML Structure and Layout

**Files:**
- Create: `polynomial-plotter.html`

**Step 1: Write basic HTML5 structure**

```html
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Interactive Polynomial Plotter</title>
    <link rel="stylesheet" href="styles.css">
    <script src="https://cdn.plot.ly/plotly-2.27.0.min.js"></script>
</head>
<body>
    <div class="container">
        <div class="plot-section">
            <div id="plot"></div>
        </div>
        <div class="controls-section">
            <div class="polynomial-manager">
                <h2>Polynomials</h2>
                <button id="add-polynomial">Add Polynomial</button>
                <div id="polynomials-list"></div>
            </div>
            <div class="range-controls">
                <h3>Plot Range</h3>
                <div class="range-inputs">
                    <label>X Min: <input type="number" id="x-min" value="-10" step="1"></label>
                    <label>X Max: <input type="number" id="x-max" value="10" step="1"></label>
                    <label>Y Min: <input type="number" id="y-min" value="-10" step="1"></label>
                    <label>Y Max: <input type="number" id="y-max" value="10" step="1"></label>
                </div>
            </div>
        </div>
    </div>
    <script src="app.js"></script>
</body>
</html>
```

**Step 2: Run local server to test HTML structure**

Run: `cd ../.. && cd .worktrees/polynomial-plotter && python -m http.server 8003`
Expected: Server starts on port 8003

**Step 3: Test HTML loads without errors**

Open: `http://localhost:8003/polynomial-plotter.html`
Expected: Page loads with basic layout (no styling yet)

**Step 4: Commit**

```bash
cd ../.. && cd .worktrees/polynomial-plotter
git add polynomial-plotter.html
git commit -m "feat: add basic HTML structure for polynomial plotter"
```

### Task 2: Implement CSS Styling with Responsive Grid Layout

**Files:**
- Create: `styles.css`

**Step 1: Write CSS for split-screen layout**

```css
* {
    margin: 0;
    padding: 0;
    box-sizing: border-box;
}

body {
    font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
    height: 100vh;
    overflow: hidden;
}

.container {
    display: grid;
    grid-template-columns: 70% 30%;
    height: 100vh;
}

.plot-section {
    padding: 20px;
    border-right: 1px solid #ddd;
    background: #f8f9fa;
}

#plot {
    width: 100%;
    height: 100%;
    border-radius: 8px;
    background: white;
    box-shadow: 0 2px 4px rgba(0,0,0,0.1);
}

.controls-section {
    padding: 20px;
    overflow-y: auto;
    background: white;
}

.polynomial-manager h2, .range-controls h3 {
    margin-bottom: 15px;
    color: #333;
}

#add-polynomial {
    background: #007bff;
    color: white;
    border: none;
    padding: 8px 16px;
    border-radius: 4px;
    cursor: pointer;
    margin-bottom: 20px;
}

#add-polynomial:hover {
    background: #0056b3;
}

.range-inputs {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 10px;
}

.range-inputs label {
    display: block;
    margin-bottom: 5px;
    color: #666;
    font-size: 14px;
}

.range-inputs input {
    width: 100%;
    padding: 4px 8px;
    border: 1px solid #ddd;
    border-radius: 4px;
}

.polynomial-card {
    border: 1px solid #ddd;
    border-radius: 8px;
    padding: 15px;
    margin-bottom: 15px;
    background: #f8f9fa;
}

.polynomial-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 10px;
}

.equation {
    font-family: 'Courier New', monospace;
    font-size: 14px;
    margin-bottom: 15px;
    padding: 8px;
    background: white;
    border-radius: 4px;
    border-left: 4px solid currentColor;
}

.remove-polynomial {
    background: #dc3545;
    color: white;
    border: none;
    padding: 4px 8px;
    border-radius: 4px;
    cursor: pointer;
    font-size: 12px;
}

.remove-polynomial:hover {
    background: #c82333;
}

.coefficient-slider {
    margin-bottom: 10px;
}

.coefficient-slider label {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 5px;
    font-size: 14px;
}

.coefficient-slider input[type="range"] {
    width: 100%;
    margin-bottom: 5px;
}

.coefficient-value {
    font-family: 'Courier New', monospace;
    font-weight: bold;
    min-width: 40px;
    text-align: right;
}

@media (max-width: 768px) {
    .container {
        grid-template-columns: 1fr;
        grid-template-rows: 60% 40%;
    }

    .plot-section {
        border-right: none;
        border-bottom: 1px solid #ddd;
    }
}
```

**Step 2: Test CSS styling**

Refresh: `http://localhost:8003/polynomial-plotter.html`
Expected: Split-screen layout with plot area on left, controls on right

**Step 3: Commit**

```bash
cd ../.. && cd .worktrees/polynomial-plotter
git add styles.css
git commit -m "feat: add responsive CSS styling with split-screen layout"
```

### Task 3: Set Up Basic JavaScript Structure and Plotly Integration

**Files:**
- Create: `app.js`

**Step 1: Write basic JavaScript structure**

```javascript
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
            coefficients: [0, 0, 0, 0, 0, 0], // a0 through a5
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
```

**Step 2: Test basic functionality**

Refresh: `http://localhost:8003/polynomial-plotter.html`
Expected: Can add polynomials, adjust coefficients, see plot updates

**Step 3: Commit**

```bash
cd ../.. && cd .worktrees/polynomial-plotter
git add app.js
git commit -m "feat: add basic JavaScript structure and Plotly integration"
```

### Task 4: Add Root Finding and Extreme Point Calculation

**Files:**
- Modify: `app.js` (add methods to PolynomialPlotter class)

**Step 1: Add root finding methods**

```javascript
// Add these methods to the PolynomialPlotter class

findRoots(coefficients) {
    const roots = [];

    // For linear polynomials
    if (this.getDegree(coefficients) === 1) {
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

        const discriminant = b * b - 4 * a * c;
        if (discriminant >= 0) {
            const sqrtDisc = Math.sqrt(discriminant);
            const root1 = (-b + sqrtDisc) / (2 * a);
            const root2 = (-b - sqrtDisc) / (2 * a);

            if (this.isInPlotRange(root1, 0)) {
                roots.push({x: root1, y: 0});
            }
            if (this.isInPlotRange(root2, 0) && Math.abs(root2 - root1) > 0.001) {
                roots.push({x: root2, y: 0});
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

bisectionMethod(coefficients, a, b, tolerance = 0.001) {
    let fa = this.evaluatePolynomial(coefficients, a);
    let fb = this.evaluatePolynomial(coefficients, b);

    if (fa * fb > 0) return null;

    while (Math.abs(b - a) > tolerance) {
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
    }

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
    for (let i = coefficients.length - 1; i >= 0; i--) {
        if (Math.abs(coefficients[i]) > 0.001) {
            return i;
        }
    }
    return 0;
}

isInPlotRange(x, y) {
    return x >= this.plotRange.xMin && x <= this.plotRange.xMax &&
           y >= this.plotRange.yMin && y <= this.plotRange.yMax;
}
```

**Step 2: Update plot method to show roots and extrema**

```javascript
// Modify the updatePlot method to include roots and extrema

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

    Plotly.newPlot('plot', traces, layout, {responsive: true});
}
```

**Step 3: Test root and extrema detection**

Refresh: `http://localhost:8003/polynomial-plotter.html`
Expected: Roots marked with X, extrema marked with triangles

**Step 4: Commit**

```bash
cd ../.. && cd .worktrees/polynomial-plotter
git add app.js
git commit -m "feat: add root finding and extreme point calculation"
```

### Task 5: Add Error Handling and Edge Cases

**Files:**
- Modify: `app.js` (add error handling)

**Step 1: Add error handling methods**

```javascript
// Add these methods to the PolynomialPlotter class

validateRangeInputs() {
    const xMin = parseFloat(document.getElementById('x-min').value);
    const xMax = parseFloat(document.getElementById('x-max').value);
    const yMin = parseFloat(document.getElementById('y-min').value);
    const yMax = parseFloat(document.getElementById('y-max').value);

    if (isNaN(xMin) || isNaN(xMax) || isNaN(yMin) || isNaN(yMax)) {
        alert('Please enter valid numbers for all range values');
        return false;
    }

    if (xMin >= xMax) {
        alert('X minimum must be less than X maximum');
        return false;
    }

    if (yMin >= yMax) {
        alert('Y minimum must be less than Y maximum');
        return false;
    }

    if (Math.abs(xMax - xMin) > 1000 || Math.abs(yMax - yMin) > 1000) {
        alert('Range values are too large. Please use smaller ranges for better performance.');
        return false;
    }

    return true;
}

handleCoefficientOverflow(coefficients) {
    // Check for very large values that might cause overflow
    const maxCoeff = Math.max(...coefficients.map(Math.abs));
    if (maxCoeff > 1000) {
        return {
            hasOverflow: true,
            message: 'Large coefficient values detected. Consider using smaller values or adjusting the plot range.'
        };
    }
    return { hasOverflow: false };
}

sanitizeEquationDisplay(coefficients) {
    // Handle edge cases in equation display
    const degree = this.getDegree(coefficients);

    if (degree === 0 && Math.abs(coefficients[0]) < 0.001) {
        return '0';
    }

    return this.formatEquation(coefficients);
}

// Update these methods with error handling

updatePlotRange() {
    if (!this.validateRangeInputs()) {
        // Reset to previous valid values
        document.getElementById('x-min').value = this.plotRange.xMin;
        document.getElementById('x-max').value = this.plotRange.xMax;
        document.getElementById('y-min').value = this.plotRange.yMin;
        document.getElementById('y-max').value = this.plotRange.yMax;
        return;
    }

    this.plotRange.xMin = parseFloat(document.getElementById('x-min').value);
    this.plotRange.xMax = parseFloat(document.getElementById('x-max').value);
    this.plotRange.yMin = parseFloat(document.getElementById('y-min').value);
    this.plotRange.yMax = parseFloat(document.getElementById('y-max').value);
    this.updatePlot();
}

updatePlot() {
    const traces = [];

    this.polynomials.forEach(polynomial => {
        // Check for coefficient overflow
        const overflowCheck = this.handleCoefficientOverflow(polynomial.coefficients);
        if (overflowCheck.hasOverflow) {
            console.warn(overflowCheck.message);
            // Still plot but with warning
        }

        try {
            const xValues = [];
            const yValues = [];

            // Generate points for the curve with error handling
            for (let x = this.plotRange.xMin; x <= this.plotRange.xMax; x += 0.1) {
                xValues.push(x);
                const y = this.evaluatePolynomial(polynomial.coefficients, x);

                // Check for NaN or Infinity
                if (!isNaN(y) && isFinite(y)) {
                    yValues.push(y);
                } else {
                    yValues.push(null); // Break in the line
                }
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

            // Add roots with error handling
            try {
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
            } catch (error) {
                console.warn('Error calculating roots:', error);
            }

            // Add extrema with error handling
            try {
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
            } catch (error) {
                console.warn('Error calculating extrema:', error);
            }

        } catch (error) {
            console.error('Error plotting polynomial:', error);
            // Add a trace indicating error
            traces.push({
                x: [],
                y: [],
                type: 'scatter',
                mode: 'text',
                text: ['Error plotting polynomial'],
                showlegend: false
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
        console.error('Error creating plot:', error);
        document.getElementById('plot').innerHTML = '<div style="padding: 20px; color: red;">Error creating plot. Please refresh the page.</div>';
    }
}

updateEquation(polynomial) {
    try {
        const equation = this.sanitizeEquationDisplay(polynomial.coefficients);
        document.getElementById(`equation-${polynomial.id}`).textContent = equation;

        // Update coefficient value displays
        for (let i = 0; i < 6; i++) {
            const valueElement = document.getElementById(`value-${polynomial.id}-${i}`);
            if (valueElement) {
                valueElement.textContent = polynomial.coefficients[i].toFixed(1);
            }
        }
    } catch (error) {
        console.error('Error updating equation:', error);
        document.getElementById(`equation-${polynomial.id}`).textContent = 'Error displaying equation';
    }
}
```

**Step 2: Test error handling**

Refresh: `http://localhost:8003/polynomial-plotter.html`
Expected: Graceful handling of invalid inputs and edge cases

**Step 3: Commit**

```bash
cd ../.. && cd .worktrees/polynomial-plotter
git add app.js
git commit -m "feat: add comprehensive error handling and edge case management"
```

### Task 6: Final Testing and Polish

**Files:**
- None (testing only)

**Step 1: Test all functionality systematically**

Test: Add multiple polynomials
Expected: Can add up to 5 polynomials with different colors

Test: Adjust coefficients with sliders
Expected: Real-time plot updates, equation display changes

Test: Range controls
Expected: Plot scales correctly, validation prevents invalid ranges

Test: Root detection
Expected: Roots marked correctly for various polynomial types

Test: Extrema detection
Expected: Max/min points marked with appropriate symbols

Test: Error handling
Expected: Graceful handling of invalid inputs, overflow conditions

Test: Responsive design
Expected: Layout adapts correctly on mobile screens

**Step 2: Kill local server**

```bash
# Kill the test server
pkill -f "python -m http.server 8003"
```

**Step 3: Final commit**

```bash
cd ../.. && cd .worktrees/polynomial-plotter
git add .
git commit -m "feat: complete interactive polynomial plotter implementation

- Add up to 5 polynomials with real-time coefficient adjustment
- Visual root and extreme point detection
- Responsive design for mobile and desktop
- Comprehensive error handling and input validation
- User-controlled plot ranges with default -10 to 10

🤖 Generated with [Claude Code](https://claude.com/claude-code)

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

**Plan complete and saved to `docs/plans/2025-12-27-interactive-polynomial-plotter.md`. Two execution options:**

**1. Subagent-Driven (this session)** - I dispatch fresh subagent per task, review between tasks, fast iteration

**2. Parallel Session (separate)** - Open new session with executing-plans, batch execution with checkpoints

**Which approach?**