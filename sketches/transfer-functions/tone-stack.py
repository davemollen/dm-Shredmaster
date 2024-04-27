from scipy import signal
import numpy as np
import matplotlib.pyplot as plt

# Set the sample rate
sample_rate = 44100  # in Hz

# Set bass and treble to a different value between 0. and 1. to see the difference in the frequency response
treble = 0.5
bass = 0.5

def get_s_domain_coefficients(bass, treble):
  # The following transfer function was derived with QsapecNG:
  # ( C1 * C2 * C3 * R2 * R3 * R4 + (1-t) * R1 * C1 * C2 * C3 * R2 * R4 + (1-t) * R1 * C1 * C2 * C3 * R2 * R3 + C1 * C2 * C3 * R2 * R3 * t*R1 ) * s^3 + ( C1 * C3 * R2 * R4 + C1 * C2 * R2 * R4 + C2 * C3 * R2 * R3 + C1 * C3 * R2 * R3 + C1 * C3 * R3 * R4 + C1 * C2 * R3 * R4 + (1-t) * R1 * C1 * C3 * R4 + (1-t) * R1 * C1 * C2 * R2 + (1-t) * R1 * C1 * C2 * R4 + (1-t) * R1 * C1 * C3 * R3 + (1-t) * R1 * C1 * C2 * R3 + C1 * C2 * R2 * t*R1 + C1 * C3 * R3 * t*R1 + C1 * C2 * R3 * t*R1 ) * s^2 + ( C2 * R2 + C1 * R2 + C3 * R3 + C2 * R3 + C1 * R3 + (1-t) * R1 * C1 ) * s
  # -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
  # ( C1 * C2 * C3 * R2 * R3 * R4 + (1-t) * R1 * C1 * C2 * C3 * R2 * R4 + (1-t) * R1 * C1 * C2 * C3 * R2 * R3 + C1 * C2 * C3 * R2 * R4 * t*R1 + C1 * C2 * C3 * R2 * R3 * t*R1 ) * s^3 + ( C2 * C3 * R2 * R4 + C1 * C3 * R2 * R4 + C1 * C2 * R2 * R4 + C2 * C3 * R2 * R3 + C1 * C3 * R2 * R3 + C1 * C3 * R3 * R4 + C1 * C2 * R3 * R4 + (1-t) * R1 * C1 * C3 * R4 + (1-t) * R1 * C1 * C2 * R2 + (1-t) * R1 * C1 * C2 * R4 + (1-t) * R1 * C1 * C3 * R3 + (1-t) * R1 * C1 * C2 * R3 + C1 * C3 * R4 * t*R1 + C1 * C2 * R2 * t*R1 + C1 * C2 * R4 * t*R1 + C1 * C3 * R3 * t*R1 + C1 * C2 * R3 * t*R1 ) * s^2 + ( C3 * R4 + C2 * R2 + C1 * R2 + C2 * R4 + C3 * R3 + C2 * R3 + C1 * R3 + (1-t) * R1 * C1 + C1 * t*R1 ) * s + 1

  # This function implements this transfer function, but with less repeated calculations.

  r1 = 22000.
  r2 = 100000. * bass
  r3 = 1000.
  r4 = 6800.

  c1 = 2.2e-8
  c2 = 2.2e-7
  c3 = 2.2e-8
  c1c2 = c1 * c2
  c1c3 = c1 * c3
  c2c3 = c2 * c3
  c1c2c3 = c1c2 * c3

  r1_a = treble * r1
  r1_b = (1. - treble) * r1

  b0 = c1c2c3 * r2 * r3 * r4 + r1_a * c1c2c3 * r2 * r4 + r1_a * c1c2c3 * r2 * r3 + c1c2c3 * r2 * r3 * r1_b
  b1 = c1c3 * r2 * r4 + c1c2 * r2 * r4 + c2c3 * r2 * r3 + c1c3 * r2 * r3 + c1c3 * r3 * r4 + c1c2 * r3 * r4 + r1_a * c1c3 * r4 + r1_a * c1c2 * r2 + r1_a * c1c2 * r4 + r1_a * c1c3 * r3 + r1_a * c1c2 * r3 + c1c2 * r2 * r1_b + c1c3 * r3 * r1_b + c1c2 * r3 * r1_b
  b2 = c2 * r2 + c1 * r2 + c3 * r3 + c2 * r3 + c1 * r3 + r1_a * c1
  b3 = 0.
  a0 = b0 + c1c2c3 * r2 * r4 * r1_b
  a1 = b1 + c2c3 * r2 * r4 + c1c3 * r4 * r1_b + c1c2 * r4 * r1_b 
  a2 = c3 * r4 + c2 * r2 + c1 * r2 + c2 * r4 + c3 * r3 + c2 * r3 + c1 * r3 + r1_a * c1 + c1 * r1_b
  a3 = 1.
  
  return ([b0, b1, b2, b3], [a0, a1, a2, a3])

# Get the s-domain coefficients
num, den = get_s_domain_coefficients(bass, treble)
print("s-domain coefficients:", (num, den))

# Apply the bilinear transform
b, a = signal.bilinear(num, den, fs=sample_rate)
print("z-domain coefficients:", (list(b), list(a)))

# Get the frequency response
w,h = signal.freqz(b, a, 2**20)
w = w * sample_rate / (2 *np.pi)

# Plot the frequency response
fig = plt.figure()
plt.title('Digital filter frequency response')
plt.semilogx(w, 20 * np.log10(abs(h)), 'b')
plt.ylabel('magnitude [dB]')
plt.xlabel('frequency [Hz]')
plt.grid()
plt.axis('tight')
plt.xlim([10, 20000])
plt.ylim([-18, 0])
plt.show()
