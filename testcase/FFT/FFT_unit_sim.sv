module FFT_unit_sim();
        logic clk, nrst, calc_fin, is_out;
    logic [31:0] data[0:63];
    logic [31:0] data_in;
    logic [31:0] data_out;
    logic [5:0] addr;
    logic [5:0] cnt;
    logic state;
    logic [31:0] w [0:31];
    logic [4:0] bit_rev [0:31];
    logic [4:0] cnt_r;

    assign data_in = data[addr];

    initial begin
        $readmemb("roter1.txt", w);
        $readmemb("bit_rev.txt", bit_rev);
    end

    FFT_unit FFT_unit_inst(
        .clk(clk),
        .nrst(nrst),
        .data_in(data_in),
        .w(w[bit_rev[cnt_r]]),
        .data_out(data_out),
        .is_out(is_out)
    );

    initial begin
        clk = 0;
        nrst = 0;
        cnt = 0;
        addr = 0;
        state = 0;
        cnt_r = 0;
        $readmemb("stage5.txt", data);
        #10 nrst = 1;
    end
    always #5 clk = ~clk;

    always @(posedge clk) begin
        if (!nrst) begin
            addr <= 0;
            cnt <= 0;
            state <= 0;
            cnt_r <= 0;
        end
        else begin
            if (is_out) begin
                state <= 1;
            end
            if (state) begin
                $display("%b", data_out);
                cnt <= cnt + 1;
                if (cnt == 63) begin
                    $finish;
                end
            end
            if (addr < 64) begin
                addr <= addr + 1;
            end
            cnt_r <= cnt_r + 1;
        end
    end

endmodule