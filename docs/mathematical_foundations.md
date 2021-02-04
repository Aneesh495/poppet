# Poppet Mathematical Foundations

This document details the governing differential equations, reliability probability models, and discrete algorithms underlying the Poppet control system.

## Fluid Transient & Joukowsky Water Hammer Equation

When a high-speed poppet valve closes in 35 milliseconds, the rapid deceleration of dense hydraulic fluid creates an acoustic compression wave that propagates upstream through the piping network.

The instantaneous peak acoustic pressure rise \(\Delta P\) is governed by the classical Joukowsky equation:

$$\Delta P = \rho \cdot c \cdot \Delta v$$

Where:
- \(\rho\): Fluid density (for ISO VG 32 mineral oil, \(\rho \approx 860.0 \text{ kg/m}^3\))
- \(c\): Speed of sound in the liquid-pipe elastic system (m/s)
- \(\Delta v\): Change in mean fluid flow velocity (m/s)

The acoustic wave propagation velocity \(c\) in elastic steel piping is expressed by:

$$c = \sqrt{\frac{K / \rho}{1 + \left(\frac{K}{E}\right)\left(\frac{D}{e}\right) c_1}}$$

Where:
- \(K\): Bulk modulus of the hydraulic fluid (\(\approx 1.6 \times 10^9 \text{ Pa}\))
- \(E\): Young's modulus of pipe material (for duplex stainless steel, \(E \approx 200 \times 10^9 \text{ Pa}\))
- \(D\): Pipe internal diameter (m)
- \(e\): Pipe wall thickness (m)
- \(c_1\): Pipe constraint factor (\(c_1 = 1 - \nu^2\) for anchored conduits)

For typical subsea manifold tubing, \(c \approx 1350 \text{ m/s}\). Rapid poppet closure arresting a flow velocity of \(3.2 \text{ m/s}\) generates an instantaneous transient pressure spike:

$$\Delta P = (860.0) \cdot (1350.0) \cdot (3.2) = 3.715 \times 10^6 \text{ Pa} \approx 37.15 \text{ bar}$$

Poppet incorporates this surge allowance directly into its high-high trip setpoint calculations to prevent spurious acoustic tripping.

## Hydraulic Orifice Flow Rate & Cavitation Number

Volumetric flow rate through the cartridge poppet seat is derived from Bernoulli's principle with vena contracta discharge coefficients:

$$Q = C_d \cdot A(x) \cdot \sqrt{\frac{2 \Delta P}{\rho}}$$

Where:
- \(Q\): Volumetric flow rate (\(\text{m}^3\text{/s}\))
- \(C_d\): Discharge coefficient (\(\approx 0.62\) for sharp-edged poppet orifices)
- \(A(x)\): Effective curtain flow area as a function of stroke position \(x\) (\(A(x) = \pi D x \sin \theta\))
- \(\theta\): Poppet seat bevel angle (\(45^\circ\))
- \(\Delta P\): Pressure drop across the valve seat (\(P_1 - P_2\))

To prevent vapor cavity collapse and seat pitting, the cavitation index \(\sigma\) is continuously computed:

$$\sigma = \frac{P_2 - P_v}{P_1 - P_2}$$

Where \(P_v\) is fluid vapor pressure. When \(\sigma < \sigma_{\text{incipient}} \approx 0.25\), Poppet throttles pilot dither to avoid micro-jet pitting.

## Nitrogen Accumulator Isentropic Gas Expansion

The hydraulic power unit utilizes high-pressure piston accumulators pre-charged with gaseous nitrogen (\(\text{N}_2\)). During rapid emergency discharge, heat transfer across bottle walls is negligible, governing gas expansion by the isentropic relation:

$$P_0 V_0^\gamma = P_1 V_1^\gamma = P(t) V(t)^\gamma$$

Where:
- \(\gamma\): Ratio of specific heats for nitrogen (\(\gamma \approx 1.4\))
- \(P_0, V_0\): Initial pre-charge pressure and gas volume at ambient temperature
- \(P_1, V_1\): Working pressure after accumulator charging
- \(P(t), V(t)\): Transient pressure and volume during fluid evacuation

The available hydraulic fluid volume \(\Delta V_{\text{fluid}}\) discharged between pressures \(P_1\) and \(P_2\) is:

$$\Delta V_{\text{fluid}} = V_0 \left[ \left(\frac{P_0}{P_2}\right)^{1/\gamma} - \left(\frac{P_0}{P_1}\right)^{1/\gamma} \right]$$

## Reliability & Safety Availability (Markov 2oo3 Architecture)

The probability of failure on demand (\(\text{PFD}_{\text{avg}}\)) for a 2-out-of-3 voted sensing system with proof test interval \(T_1\) and mean time to repair \(\text{MTTR}\) is:

$$\text{PFD}_{\text{avg}}^{2oo3} = 3 (\lambda_D^D)^2 \cdot \frac{T_1^2}{3} + 3 (\lambda_D^D) (\lambda_D^U) \cdot T_1 \cdot \text{MTTR} + \beta \lambda_D \cdot \frac{T_1}{2}$$

Where:
- \(\lambda_D\): Total dangerous failure rate of each pressure transmitter
- \(\lambda_D^D\): Dangerous detected failure rate (isolated via diagnostic deviation alarms)
- \(\lambda_D^U\): Dangerous undetected failure rate
- \(\beta\): Common cause failure coupling factor (\(\approx 0.05\) for diverse sensor tapping)

Because detected failures (\(\lambda_D^D\)) trigger automatic degradation to 1oo2 voting rather than spurious shutdown, the spurious trip rate is minimized:

$$\text{STR}^{2oo3} = 6 \lambda_S^2 \cdot \text{MTTR}$$

Where \(\lambda_S\) is the safe failure rate per transmitter channel.

## Fixed-Point PID Algorithm & Anti-Windup

Fixed-point math operates in \(Q15.16\) representation within Funge-Space. The discrete proportional-integral-derivative regulator utilizes backward Euler integration with anti-windup clamping:

$$e[k] = r[k] - y[k]$$

$$u_P[k] = K_p \cdot e[k]$$

$$u_I[k] = u_I[k-1] + \frac{K_i \cdot T_s}{2} (e[k] + e[k-1])$$

$$u_D[k] = \frac{K_d}{T_s + N} (e[k] - e[k-1])$$

$$u[k] = \text{clamp}(u_P[k] + u_I[k] + u_D[k], u_{\min}, u_{\max})$$

Where \(N\) is the derivative noise filter coefficient, and \(T_s\) is the discrete sample interval.
