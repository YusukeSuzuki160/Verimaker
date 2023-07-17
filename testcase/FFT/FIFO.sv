module FIFO(
    input clk, nrst, rden, wren,
    input [31:0] data_in1, data_in2,
    output [31:0] data_out
    );
    logic [5:0] read_addr, write_addr;
    logic [31:0] mem [0:63];
    logic write_fin;

    assign data_out = mem[read_addr];

    always_ff @(posedge clk, negedge nrst) begin
        if (!nrst) begin
            read_addr <= 0;
            write_addr <= 0;
            write_fin <= 0;
        end
        else begin
            if (wren && !write_fin) begin
                mem[write_addr] <= data_in1;
                mem[write_addr + 1] <= data_in2;
                if (write_addr == 62) begin
                    write_addr <= 0;
                    write_fin <= 1;
                end
                else begin
                    write_addr <= write_addr + 2;
                end
            end
            if (rden) begin
                read_addr <= read_addr + 1;
            end
        end
    end
endmodule