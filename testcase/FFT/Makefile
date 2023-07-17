RUN = iverilog -g2009
SRCS = /home/meltpoint/verirunner/testcase/FFT/shift_reg.sv /home/meltpoint/verirunner/testcase/FFT/butt2.sv /home/meltpoint/verirunner/testcase/FFT/FIFO.sv /home/meltpoint/verirunner/testcase/FFT/FFT_unit.sv /home/meltpoint/verirunner/testcase/FFT/reoder.sv /home/meltpoint/verirunner/testcase/FFT/top.sv /home/meltpoint/verirunner/testcase/FFT/top_sim.sv /home/meltpoint/verirunner/testcase/FFT/FFT_unit_sim.sv
TARGET = a.out

$(TARGET): $(SRCS)
	$(RUN) $(SRCS)

.PHONY: clean

clean:
	rm -f $(TARGET)

.PHONY: clean_all

clean_all:
	rm -f $(TARGET)
	find . -name "*.vcd" -delete

.PHONY: run

run:
	$(RUN) $(SRCS)
	./$(TARGET)
