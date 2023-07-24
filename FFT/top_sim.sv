module top_sim();
    logic clk, nrst, calc_fin, is_out;
    logic [15:0] data[0:63];
    logic [31:0] data_in;
    logic [31:0] data_out;
    logic [5:0] addr;
    logic [5:0] cnt;
    logic state;

    assign data_in = {data[addr], 16'b0};

    top top_inst(
        .clk(clk),
        .nrst(nrst),
        .data_in(data_in),
        .data_out(data_out),
        .is_out(is_out)
    );

    initial begin
        clk = 0;
        nrst = 0;
        cnt = 0;
        $readmemb("input.txt", data);
        #10 nrst = 1;
    end
    
    always #5 clk = ~clk;

    always @(posedge clk) begin
        if (!nrst) begin
            addr <= 0;
        end
        else begin
            if (is_out) begin
                $display("%b", data_out);
                cnt <= cnt + 1;
                if (cnt == 63) begin
                    $finish;
                end
            end
            if (addr < 64) begin
                addr <= addr + 1;
            end
        end
    end

endmodule