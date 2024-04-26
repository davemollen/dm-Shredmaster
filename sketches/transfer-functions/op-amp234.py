from scipy import signal
import numpy as np
import matplotlib.pyplot as plt
from enum import Enum

# Set the sample rate
sample_rate = 44100  # in Hz

# Predefined op-amp configurations
class OpAmp(Enum):
  TWO = 0
  THREE = 1
  FOUR = 3
  FOUR_WITH_BRILLIANCE = 2

# Change the selected op_amp to see the frequency response for other op_amps
selected_op_amp = OpAmp.TWO

# Use predefined s-domain coefficients
# The s-domain coefficients are generated with this op-amp tool: 
# http://sim.okawa-denshi.jp/en/opampkeisan.htm
match selected_op_amp:
  case OpAmp.TWO:
    num = [-2594706.7981318, 0.]
    den = [1., 33082.511676181, 56113901.343681]
    ylim = [5, 40]
  case OpAmp.THREE:
    num = [-9671.1798839458, 0.]
    den = [1., 2162.8275013188, 199817.76619723]
    ylim = [-24, 16]
  case OpAmp.FOUR:
    num = [-10000., 0.]
    den = [1., 10045.454545455, 454545.45454545]
    ylim = [0, -24]
  case OpAmp.FOUR_WITH_BRILLIANCE:
    num = [-21276.595744681, 0.] 
    den = [1., 21322.050290135, 967117.98839458]
    ylim = [0, -24]
    
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
plt.ylim(ylim)
plt.show()
