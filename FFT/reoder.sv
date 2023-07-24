module reorder (
    input clk,
    nrst,
    input [31:0] data_in,
    output [31:0] data_out,
    output logic is_out
);

  logic [31:0] data_out1, data_out2;
  logic rden, wren;
  logic [5:0] cnt;
  logic [5:0] bit_rev[0:63];

  assign bit_rev[0]  = 6'b000000;
  assign bit_rev[1]  = 6'b100000;
  assign bit_rev[2]  = 6'b010000;
  assign bit_rev[3]  = 6'b110000;
  assign bit_rev[4]  = 6'b001000;
  assign bit_rev[5]  = 6'b101000;
  assign bit_rev[6]  = 6'b011000;
  assign bit_rev[7]  = 6'b111000;
  assign bit_rev[8]  = 6'b000100;
  assign bit_rev[9]  = 6'b100100;
  assign bit_rev[10] = 6'b010100;
  assign bit_rev[11] = 6'b110100;
  assign bit_rev[12] = 6'b001100;
  assign bit_rev[13] = 6'b101100;
  assign bit_rev[14] = 6'b011100;
  assign bit_rev[15] = 6'b111100;
  assign bit_rev[16] = 6'b000010;
  assign bit_rev[17] = 6'b100010;
  assign bit_rev[18] = 6'b010010;
  assign bit_rev[19] = 6'b110010;
  assign bit_rev[20] = 6'b001010;
  assign bit_rev[21] = 6'b101010;
  assign bit_rev[22] = 6'b011010;
  assign bit_rev[23] = 6'b111010;
  assign bit_rev[24] = 6'b000110;
  assign bit_rev[25] = 6'b100110;
  assign bit_rev[26] = 6'b010110;
  assign bit_rev[27] = 6'b110110;
  assign bit_rev[28] = 6'b001110;
  assign bit_rev[29] = 6'b101110;
  assign bit_rev[30] = 6'b011110;
  assign bit_rev[31] = 6'b111110;
  assign bit_rev[32] = 6'b000001;
  assign bit_rev[33] = 6'b100001;
  assign bit_rev[34] = 6'b010001;
  assign bit_rev[35] = 6'b110001;
  assign bit_rev[36] = 6'b001001;
  assign bit_rev[37] = 6'b101001;
  assign bit_rev[38] = 6'b011001;
  assign bit_rev[39] = 6'b111001;
  assign bit_rev[40] = 6'b000101;
  assign bit_rev[41] = 6'b100101;
  assign bit_rev[42] = 6'b010101;
  assign bit_rev[43] = 6'b110101;
  assign bit_rev[44] = 6'b001101;
  assign bit_rev[45] = 6'b101101;
  assign bit_rev[46] = 6'b011101;
  assign bit_rev[47] = 6'b111101;
  assign bit_rev[48] = 6'b000011;
  assign bit_rev[49] = 6'b100011;
  assign bit_rev[50] = 6'b010011;
  assign bit_rev[51] = 6'b110011;
  assign bit_rev[52] = 6'b001011;
  assign bit_rev[53] = 6'b101011;
  assign bit_rev[54] = 6'b011011;
  assign bit_rev[55] = 6'b111011;
  assign bit_rev[56] = 6'b000111;
  assign bit_rev[57] = 6'b100111;
  assign bit_rev[58] = 6'b010111;
  assign bit_rev[59] = 6'b110111;
  assign bit_rev[60] = 6'b001111;
  assign bit_rev[61] = 6'b101111;
  assign bit_rev[62] = 6'b011111;
  assign bit_rev[63] = 6'b111111;


  logic [31:0] robuf[0:63];



  assign data_out = robuf[bit_rev[cnt]];

  always_ff @(posedge clk) begin
    if (!nrst) begin
      rden <= 0;
      wren <= 0;
      cnt <= 0;
      is_out <= 0;
    end else begin
      if (cnt == 63) begin
        wren   <= 1;
        is_out <= 1;
      end
      if (wren == 1) begin
        rden <= 1;
      end else begin
        rden <= 0;
        robuf[cnt] <= data_in;
      end
      cnt <= cnt + 1;
    end
  end
endmodule
