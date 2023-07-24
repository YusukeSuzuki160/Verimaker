module shift_reg(
    input clk, nrst,
    input [31:0] data_in,
    output [31:0] data_out1, data_out2
    );
    parameter read_add1 = 0;
    parameter read_add2 = 32;

    logic [31:0] shreg [0:63];

    assign data_out1 = shreg[read_add1];
    assign data_out2 = shreg[read_add2];

    always_ff @(posedge clk) begin
        if (nrst == 1'b0) begin
            for (int i = 0; i < 64; i++) begin
                shreg[i] <= 0;
            end
        end
        else begin
            for (int i = 0; i < 63; i = i + 1) begin
                shreg[i] <= shreg[i+1];
            end
            shreg[63] <= data_in;
        end
    end


endmodule