###############################################################################
# This script calculates the baud value for UART
# The baud value is calculated as follows:
# baud_rate = (sel_clk / CD * (BDIV + 1))
# where:
# sel_clk = 100 MHz
# CD = Values between 1 and 65535
# BDIV = Values between 4 and 255
#
# To find the best CD and BDIV values, we need to find the best combination of CD and BDIV
# that will reach us to our desired baud rate.
#
# The desired baud rate is 115200
#
# I found for some reason that my calulations had the baud rate
# in the 0.[baud rate], so we are looking for the closest value to 0.1152

import math

# Find the best CD and BDIV values for 868 (if there are any)
# where:
# CD is between 1 and 65535
# BDIV is between 4 and 255
# Just go through and find what combination of CD and BDIV + 1 will get us to 868
# Please generate me a function that will try to find the best value of CD and BDIV + 1 that will get us to 868

def find_best_cd_bdiv(target_baud=868):
    best_cd = None
    best_bdiv = None
    best_diff = float('inf')

    for cd in range(1, 65536):
        for bdiv in range(4, 256):
            baud = cd * (bdiv + 1)
            diff = abs(baud - target_baud)
            if diff < best_diff:
                best_diff = diff
                best_cd = cd
                best_bdiv = bdiv

    return best_cd, best_bdiv + 1, best_diff

cd, bdiv_plus_1, diff = find_best_cd_bdiv()
print(f"Best CD: {cd}, Best BDIV + 1: {bdiv_plus_1}, Difference: {diff}")
