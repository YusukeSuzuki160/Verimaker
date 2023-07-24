RUN = iverilog -g2009
SRCS = /home/meltpoint/verirunner/FFT/top.sv /home/meltpoint/verirunner/FFT/FFT_unit.sv /home/meltpoint/verirunner/FFT/shift_reg.sv /home/meltpoint/verirunner/FFT/butt2.sv /home/meltpoint/verirunner/FFT/FIFO.sv /home/meltpoint/verirunner/FFT/reoder.sv /home/meltpoint/verirunner/FFT/top_sim.sv
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
