import matplotlib.pyplot as plt
from matplotlib.lines import Line2D
import numpy as np

# Δdelay values in nanoseconds
delay = [
    0, 0.224, 0.2464, 0.2716, 0.294, 0.322, 0.3444, 0.3696, 0.392, 0.42, 
    0.4424, 0.4676, 0.49, 0.518, 0.5404, 0.5656, 0.588, 0.616, 0.6356, 0.6636, 
    0.686, 0.714, 0.7336, 0.7588, 0.784, 0.8092, 0.8316, 0.8568, 0.882, 0.9072, 
    0.9296, 0.9548, 0.9772, 1.0052, 1.0276, 1.0528, 1.0752, 1.1032, 1.1256, 1.1508, 
    1.1732, 1.2012, 1.2236, 1.2488, 1.2712, 1.2992, 1.3188, 1.344, 1.3692, 1.3972, 
    1.4168, 1.442, 1.4672, 1.4924, 1.5148, 1.54, 1.5652, 1.5904, 1.6128, 1.638, 
    1.6604, 1.6884, 1.7108, 1.736, 1.7584, 1.7864, 1.8088, 1.834, 1.8564, 1.8844, 
    1.9068, 1.932, 1.9544, 1.9824, 2.002, 2.03, 2.0524, 2.0804, 2.1, 2.1252, 
    2.1504, 2.1756, 2.198, 2.2232, 2.2484, 2.2736, 2.296, 2.3212, 2.3436, 2.3716, 
    2.394, 2.4192, 2.4416, 2.4696, 2.492, 2.5172, 2.5396, 2.5676, 2.59, 2.6152, 
    2.6376, 2.6656, 2.6852, 2.7132, 2.7356, 2.7636, 2.7832, 2.8084, 2.8336, 2.8588, 
    2.8812, 2.9064, 2.9316, 2.9568, 2.9792, 3.0044, 3.0268, 3.0548, 3.0772, 3.1024, 
    3.1248, 3.1528, 3.1752, 3.2004, 3.2228, 3.2508, 3.2732, 3.2984, 3.3208, 3.3488, 
    3.3684, 3.3964, 3.4188, 3.4468, 3.4664, 3.4916, 3.5168, 3.5448, 3.5644, 3.5896, 
    3.6148, 3.64, 3.6624, 3.6876, 3.71, 3.738, 3.7604, 3.7856, 3.808, 3.836, 
    3.8584, 3.8836, 3.906, 3.934, 3.9564, 3.9816, 4.004, 4.032, 4.0516, 4.0796, 
    4.102, 4.13, 4.1496, 4.1748, 4.2, 4.228, 4.2476, 4.2728, 4.298, 4.3232, 
    4.3456, 4.3708, 4.3932, 4.4212, 4.4436, 4.4688, 4.4912, 4.5192, 4.5416, 4.5668, 
    4.5892, 4.6172, 4.6396, 4.6648, 4.6872, 4.7152, 4.7348, 4.7628, 4.7852, 4.8132, 
    4.8328, 4.858, 4.8832, 4.9112, 4.9308, 4.956, 4.9812, 5.0064, 5.0288, 5.054, 
    5.0764, 5.1044, 5.1268, 5.152, 5.1744, 5.2024, 5.2248, 5.25, 5.2724, 5.3004, 
    5.3228, 5.348, 5.3704, 5.3984, 5.418, 5.446, 5.4684, 5.4964, 5.516, 5.5412, 
    5.5664, 5.5944, 5.614, 5.6392, 5.6644, 5.6896, 5.712, 5.7372, 5.7596, 5.7876, 
    5.81, 5.8352, 5.8576, 5.8856, 5.908, 5.9332, 5.9556, 5.9836, 6.006, 6.0312, 
    6.0536, 6.0816, 6.1012, 6.1292, 6.1516, 6.1796, 6.1992, 6.2244, 6.2496, 6.2776, 
    6.2972, 6.3196, 6.3392
]
# Total output counts
dout_count = list(range(len(delay)))


# Define new delays extending beyond the last known delay
end_delays = np.arange(6.3392, 7, 0.0028)  # Adjust the range as necessary

# Append new delays to the existing array
delay = np.append(delay, end_delays)

# Extend the outputs array by repeating the last output value (252) for the new delays
last_output = 252  # Fixed output value for extended range
new_outputs = np.full(end_delays.shape, last_output)
dout_count = np.append(dout_count, new_outputs)


# Create the plot
plt.figure(figsize=(300, 150))
plt.plot(delay, dout_count, drawstyle='steps-post', linestyle='-', color='b')

for i, (x, y) in enumerate(zip(delay, dout_count)):
    if x <= 6.3392:
        plt.text(x + 0.01, y, str(y), color="orange", fontsize=15, ha='center', va='bottom')
        plt.text(x, y - 1.2, str(x), color="red", fontsize=12, ha='center', va='top')

xticks = np.arange(min(delay), max(delay) + 0.028, 0.028)
plt.xticks(ticks=xticks, labels=[f'{tick:.3f}' for tick in xticks], rotation=45, fontsize=45)

yticks = np.arange(min(dout_count), max(dout_count) + 5, 5)
plt.yticks(ticks=yticks, labels=[f'{tick:.3f}' for tick in yticks], fontsize=45)


plt.title('Graph of Data Output vs Delay of tdc_64', fontsize=80)
plt.xlabel('ΔDelay (ns)', fontsize=60)
plt.ylabel('Output Counts', fontsize=60)

plt.grid(True)

# Creating custom legends for the text annotations
# legend_elements = [Line2D([0], [0], color='orange', lw=20, label='Output Counts', size=40),
#                    Line2D([0], [0], color='red', lw=20, label='Delay Values', size=40)]

# Adding the legend to the plot
# plt.legend(handles=legend_elements, loc='upper right', fontsize='large', title='Legend', title_fontsize='')

# Save data
plt.savefig('Data_Outputs_vs_Delay_tdc_64.pdf', format='pdf', dpi=300)
