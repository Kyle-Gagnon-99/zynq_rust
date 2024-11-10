########################################################################################
# program_board.py
#
# This script will use XSDB to program the FPGA with the FSBL, FPGA bitstream, and then
# the custom Rust bare-metal ELF file.
########################################################################################

import argparse
import time
from xsdb import * # type: ignore

# Declare constants
FSBL_FILE = 'resources/bin/zynq_fsbl.elf'
BITSTREAM_FILE = 'resources/bin/z7_block_wrapper.bit'
IMG_FILE = 'target/zynq_target/debug/zynq_bin'

def parse_args():
    parser = argparse.ArgumentParser(prog='program_board.py',
                                     description='Program the FPGA with the FSBL, FPGA bitstream, and custom Rust bare-metal ELF file.')
    
    parser.add_argument('--fsbl', type=str, default=FSBL_FILE,
                        help='The path to the FSBL file.')
    
    parser.add_argument('--bitstream', type=str, default=BITSTREAM_FILE,
                        help='The path to the FPGA bitstream file.')
    
    parser.add_argument('--img', type=str, default=IMG_FILE,
                        help='The path to the custom Rust bare-metal ELF file.')
    
    parser.add_argument('--hw-server', type=str, default='TCP:host.docker.internal:3121',
                        help='The HW Server URL to connect to.')
    
    parser.add_argument('--debug', action='store_true', default=False,
                        help='After programming the board, start a debug session. Default is False.')
    
    parser.add_argument('--skip-programming', action='store_true', default=False,
                        help='Skip programming the board with the custom bare-metal ELF file. Useful to allow GDB to load the ELF file. Default is False.')
    
    return parser.parse_args()

def main(args=None):
    print("Programming the FPGA with the FSBL, FPGA bitstream, and custom Rust bare-metal ELF file.")
    
    # Create a session
    session = start_debug_session() # type: ignore
    
    # Connect to the HW Server
    session.connect(url=args.hw_server)
    
    # Connect to the APU's ARM Processor First Core
    session.targets(2)
    
    # First reset the board
    session.rst()
    
    # Download the FPGA bitstream to the board
    session.fpga(args.bitstream)
    
    # Wait a little for it
    time.sleep(3)
    
    # Download the FSBL to the board
    session.dow(args.fsbl)
    session.con()
    
    # Give the FSBL time to run
    time.sleep(5)
    session.stop()
    
    # Reset the board
    #session.rst()
    #session.stop()
    
    # Download the custom bare-metal ELF file to the board (if not skipping)
    if not args.skip_programming:
        session.dow(args.img)
        
        if not args.debug:
            session.con()
    
    
if __name__ == "__main__":
    args = parse_args()
    main(args=args)