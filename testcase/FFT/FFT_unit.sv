module FFT_unit(
    input clk, nrst, 
    input [31:0] data_in, w,
    output [31:0] data_out,
    output logic is_out
    );
    logic [31:0] data_out1, data_out2, data_out3, data_out4;
    logic rden, wren;
    logic [5:0] cnt;
    shift_reg sr(
        .clk(clk),
        .nrst(nrst),
        .data_in(data_in),
        .data_out1(data_out1),
        .data_out2(data_out2)
    );
    butt2 but (
        .x0(data_out1),
        .x1(data_out2),
        .W(w),
        .y0(data_out3),
        .y1(data_out4)
    );
    FIFO fifo(
        .clk(clk),
        .nrst(nrst),
        .rden(rden),
        .wren(wren),
        .data_in1(data_out3),
        .data_in2(data_out4),
        .data_out(data_out)
    );
    always_ff @(posedge clk) begin
        if(!nrst) begin
            rden <= 0;
            wren <= 0;
            cnt <= 0;
            is_out <= 0;
        end
        else begin
            if (cnt == 63) begin
                wren <= 1;
                is_out <= 1;
            end
            if (wren == 1) begin
                rden <= 1;
            end
            cnt <= cnt + 1;
        end
    end
endmodule