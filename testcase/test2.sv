module top (
    input clk,
    nrst,
    input [31:0] data_in,
    output [31:0] data_out,
    output logic is_out
);
  logic [31:0] data_stage1, data_stage2, data_stage3, data_stage4, data_stage5, data_stage6;
  logic is_out1, is_out2, is_out3, is_out4, is_out5, is_out6;
  logic [31:0] w[0:31];
  logic [4:0] cnt6;
  logic [3:0] cnt5;
  logic [2:0] cnt4;
  logic [1:0] cnt3;
  logic cnt2;
  logic nrst_stage2, nrst_stage3, nrst_stage4, nrst_stage5, nrst_stage6, nrst_ro;
  assign w[0]  = 32'b00001000000000000000000000000000;
  assign w[1]  = 32'b00000000000000000000100000000000;
  assign w[2]  = 32'b00000101101010000000010110101000;
  assign w[3]  = 32'b11111010010110000000010110101000;
  assign w[4]  = 32'b00000111011001000000001100010000;
  assign w[5]  = 32'b11111100111100000000011101100100;
  assign w[6]  = 32'b00000011000100000000011101100100;
  assign w[7]  = 32'b11111000100111000000001100010000;
  assign w[8]  = 32'b00000111110110010000000110010000;
  assign w[9]  = 32'b11111110011100000000011111011001;
  assign w[10] = 32'b00000100011100100000011010100111;
  assign w[11] = 32'b11111001010110010000010001110010;
  assign w[12] = 32'b00000110101001110000010001110010;
  assign w[13] = 32'b11111011100011100000011010100111;
  assign w[14] = 32'b00000001100100000000011111011001;
  assign w[15] = 32'b11111000001001110000000110010000;
  assign w[16] = 32'b00000111111101100000000011001001;
  assign w[17] = 32'b11111111001101110000011111110110;
  assign w[18] = 32'b00000101000100110000011000101111;
  assign w[19] = 32'b11111001110100010000010100010011;
  assign w[20] = 32'b00000111000011100000001111000101;
  assign w[21] = 32'b11111100001110110000011100001110;
  assign w[22] = 32'b00000010010100110000011110101000;
  assign w[23] = 32'b11111000010110000000001001010011;
  assign w[24] = 32'b00000111101010000000001001010011;
  assign w[25] = 32'b11111101101011010000011110101000;
  assign w[26] = 32'b00000011110001010000011100001110;
  assign w[27] = 32'b11111000111100100000001111000101;
  assign w[28] = 32'b00000110001011110000010100010011;
  assign w[29] = 32'b11111010111011010000011000101111;
  assign w[30] = 32'b00000000110010010000011111110110;
  assign w[31] = 32'b11111000000010100000000011001001;

  reorder ro (
      .clk(clk),
      .nrst(nrst_ro),
      .data_in(data_stage6),
      .data_out(data_out),
      .is_out(is_out)
  );

  always_ff @(posedge clk) begin
    if (!nrst) begin
      cnt6 <= 0;
      cnt5 <= 0;
      cnt4 <= 0;
      cnt3 <= 0;
      cnt2 <= 0;
      nrst_stage2 <= 0;
      nrst_stage3 <= 0;
      nrst_stage4 <= 0;
      nrst_stage5 <= 0;
      nrst_stage6 <= 0;
      nrst_ro <= 0;
    end else begin
      if (is_out1) begin
        nrst_stage2 <= 1;
      end
      if (is_out2) begin
        nrst_stage3 <= 1;
      end
      if (is_out3) begin
        nrst_stage4 <= 1;
      end
      if (is_out4) begin
        nrst_stage5 <= 1;
      end
      if (is_out5) begin
        nrst_stage6 <= 1;
      end
      if (is_out6) begin
        nrst_ro <= 1;
      end
      if (nrst_stage2) begin
        cnt2 <= cnt2 + 1;
      end
      if (nrst_stage3) begin
        cnt3 <= cnt3 + 1;
      end
      if (nrst_stage4) begin
        cnt4 <= cnt4 + 1;
      end
      if (nrst_stage5) begin
        cnt5 <= cnt5 + 1;
      end
      if (nrst_stage6) begin
        cnt6 <= cnt6 + 1;
      end
    end
  end
endmodule
