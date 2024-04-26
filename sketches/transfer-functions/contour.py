from scipy import signal
import numpy as np
import matplotlib.pyplot as plt

# Set the sample rate
sample_rate = 44100  # in Hz

# Set contour to a different value between 0. and 1. to see the difference in the frequency response
contour = 1.

def get_s_domain_coefficients(contour):
  # The following transfer function was derived with Ngspice:
  # ( (1-m)*R4 * C1 * C2 * C3 * R3 * m*R4 + (1-m)*R4 * C1 * C2 * C3 * R2 * m*R4 + C1 * C2 * C3 * R2 * R3 * m*R4 ) * s^3 + ( (1-m)*R4 * C2 * C3 * m*R4 + (1-m)*R4 * C1 * C3 * R3 + C1 * C2 * R3 * m*R4 + (1-m)*R4 * C1 * C3 * R2 + C1 * C2 * R2 * m*R4 + C1 * C3 * R2 * R3 ) * s^2 + ( (1-m)*R4 * C3 + C2 * m*R4 + C1 * R3 + C1 * R2 ) * s
  # -----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
  # ( (1-m)*R4 * C1 * C2 * C3 * R3 * m*R4 + (1-m)*R4 * C1 * C2 * C3 * R2 * m*R4 + C1 * C2 * C3 * R2 * R3 * m*R4 + (1-m)*R4 * C1 * C2 * C3 * R1 * R3 + C1 * C2 * C3 * R1 * R3 * m*R4 + (1-m)*R4 * C1 * C2 * C3 * R1 * R2 + C1 * C2 * C3 * R1 * R2 * m*R4 + C1 * C2 * C3 * R1 * R2 * R3 ) * s^3 + ( (1-m)*R4 * C2 * C3 * m*R4 + (1-m)*R4 * C1 * C3 * R3 + C1 * C2 * R3 * m*R4 + (1-m)*R4 * C1 * C3 * R2 + C1 * C2 * R2 * m*R4 + C2 * C3 * R2 * m*R4 + C1 * C3 * R2 * R3 + (1-m)*R4 * C2 * C3 * R1 + C2 * C3 * R1 * m*R4 + C1 * C2 * R1 * R3 + C1 * C3 * R1 * R3 + C1 * C2 * R1 * R2 + C1 * C3 * R1 * R2 + C2 * C3 * R1 * R2 ) * s^2 + ( (1-m)*R4 * C3 + C2 * m*R4 + C1 * R3 + C1 * R2 + C3 * R2 + C2 * R1 + C3 * R1 ) * s + 1

  # This function implements this transfer function, but with less repeated calculations.

  r1 = 100.
  r2 = 33000.
  r3 = 33000.
  r4 = 100000.
  r4_a = contour * r4
  r4_b = (1.- contour) * r4
  c1 = 1e-9
  c2 = 1e-7
  c3 = 4.7e-8

  c1c2 = c1 * c2
  c1c3 = c1 * c3
  c2c3 = c2 * c3
  c1c3r1 = c1c3 * r1
  c1c2c3 = c1c2 * c3
  c1c3r2 = c1c3 * r2
  c2c3r1 = c2c3 * r1
  c1c2r1 = c1c2 * r1
  c1c2c3r1 = c1c2c3 * r1
  c1c2c3r1r3 = c1c2c3r1 * r3
  c1c2c3r1r2 = c1c2c3r1 * r2
  r3r4_a = r3 * r4_a
  r2r4_a = r2 * r4_a

  b0 = r4_b * c1c2c3 * r3 * r4_a + r4_b * c1c2c3 * r2r4_a + c1c2c3 * r2r4_a * r3
  b1 = r4_b * c2c3 * r4_a + r4_b * c1c3 * r3 + c1c2 * r3r4_a + r4_b * c1c3r2 + c1c2 * r2r4_a + c1c3r2 * r3
  b2 = r4_b * c3 + c2 * r4_a + c1 * r3 + c1 * r2
  b3 = 1
  a0 = b0 + r4_b * c1c2c3r1r3 + c1c2c3r1 * r3r4_a + r4_b * c1c2c3r1r2 + c1c2c3r1r2 * r4_a + c1c2c3r1r2 * r3
  a1 = b1 + c2c3 * r2r4_a + r4_b * c2c3r1 + c2c3r1 * r4_a + c1c2r1 * r3 + c1c3r1 * r3 + c1c2r1 * r2 + c1c3r1 * r2 + c2c3r1 * r2
  a2 = b2 + c3 * r2 + c2 * r1 + c3 * r1
  a3 = 1.

  return ([b0,b1,b2,b3], [a0,a1,a2,a3])

# Get the s-domain coefficients
num, den = get_s_domain_coefficients(contour)
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
plt.xlim([1, 20000])
plt.ylim([-32, 0])
plt.show()
