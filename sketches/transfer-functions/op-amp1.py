from scipy import signal
import numpy as np
import matplotlib.pyplot as plt
from enum import Enum

# Set the sample rate
sample_rate = 44100  # in Hz

# //////////////////////////////
# PREDEFINED S-DOMAIN COEFFICIENTS

# Predefined gain pot values
class GainPotValue(float, Enum):
  ONE = 1.
  HUNDRED = 100.
  THOUSAND = 1000.
  FIFTY_THOUSAND = 50000.
  HUNDRED_THOUSAND = 100000.

# Change the gain_pot_value to see the difference in the frequency response
gain_pot_value = GainPotValue.THOUSAND

# Use predefined s-domain coefficients
match gain_pot_value:
  case GainPotValue.ONE:
    num = [1., 10003036750.484, 64474532559639]
    den = [1., 10000006447.453, 64474532559639]
  case GainPotValue.HUNDRED:
    num = [1., 103036750.48356, 644745325596.39]
    den = [1., 100006447.45326, 644745325596.39]
  case GainPotValue.THOUSAND:
    num = [1., 13036750.483559, 64474532559.639]
    den = [1., 10006447.453256, 64474532559.639]
  case GainPotValue.FIFTY_THOUSAND:
    num = [1., 3236750.483559, 1289490651.1928]
    den = [1., 206447.45325596, 1289490651.1928]
  case GainPotValue.HUNDRED_THOUSAND:
    num = [1., 3136750.483559, 644745325.59639]
    den = [1., 106447.45325596, 644745325.59639]

# Apply the bilinear transform
b, a = signal.bilinear(num, den, fs=sample_rate)

# Get the frequency response
w,h = signal.freqz(b, a, 2**20)
w = w * sample_rate / (2 *np.pi)

# Plot the frequency response
fig = plt.figure(1)
plt.title('Digital filter frequency response')
plt.semilogx(w, 20 * np.log10(abs(h)), 'b')
plt.ylabel('magnitude [dB]')
plt.xlabel('frequency [Hz]')
plt.grid()
plt.axis('tight')
plt.xlim([10, 20000])
plt.ylim(0, 30)

# ///////////////////////////////
# GENERATED S-DOMAIN COEFFICIENTS

def generate_s_domain_coefficients(gain_pot_value):
  r1 = 3300.
  c1 = 4.7e-8
  r2 = max(gain_pot_value, 1)
  c2 = 1e-10

  r1c1 = r1 * c1
  r2c2 = r2 * c2

  a0 = r1c1 * r2c2
  a1 = r1c1 + r2c2
  b1 = r2 * c1 + a1

  return ([a0, b1, 1.], [a0, a1, 1.])
    
# Use s-domain coefficients derived from just the distortion_pot_value
num, den = generate_s_domain_coefficients(gain_pot_value.value)
print('s-domain coefficients', (num, den))

# Apply the bilinear transform
b, a = signal.bilinear(num, den, fs=sample_rate)
print('z-domain coefficients', (list(b), list(a)))

# Get the frequency response
w,h = signal.freqz(b, a, 2**20)
w = w * sample_rate / (2 *np.pi)

# Plot the frequency response
fig2 = plt.figure(2)
plt.title('Digital filter frequency response')
plt.semilogx(w, 20 * np.log10(abs(h)), 'b')
plt.ylabel('magnitude [dB]')
plt.xlabel('frequency [Hz]')
plt.grid()
plt.axis('tight')
plt.xlim([10, 20000])
plt.ylim(0, 30)
plt.show()
