from scipy import signal
import numpy as np
import matplotlib.pyplot as plt
from enum import Enum

# Set the sample rate
sample_rate = 44100  # in Hz

# Set gain to a different value between 0. and 1. to see the difference in the frequency response
gain = 1.

def get_s_domain_coefficients(gain):
  r1 = 3300.
  c1 = 4.7e-8
  r2 = gain * 100000.
  c2 = 1e-10

  b1 = r2 * c1
  z2_a = r1 * c1
  z2_b = c2 * r2

  a0 = z2_a * z2_b
  a1 = z2_a + z2_b

  return ([-b1 / a0, 0.], [1., a1 / a0, 1. / a0])
    
# Get the s-domain coefficients
num, den = get_s_domain_coefficients(gain)
print("s-domain coefficients:", (num, den))

# Apply the bilinear transform
b, a = signal.bilinear(num, den, fs=sample_rate)
print("z-domain coefficients", (list(b), list(a)))

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
plt.ylim(0, 30)
plt.show()
