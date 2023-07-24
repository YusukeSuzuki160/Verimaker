module butt2(y0,y1,x0,x1,W);
  input [31:0] x0,x1,W;
  output[31:0] y0,y1;

  wire [31:0] x11;
  
  assign x11 = mulc(x1, W);
  assign y0 = addc( x0, x11 );
  assign y1 = subc( x0, x11 );
  
  function [31:0] addc;
    input [31:0] a, b;
    logic [15:0] yr, yi;
    begin
      yr = a[31:16] + b[31:16];
      yi = a[15:0] + b[15:0];
      addc = {yr, yi};
    end
  endfunction
  function [31:0] subc;
    input [31:0] a, b;
    logic [15:0] yr, yi;
    begin
      yr = a[31:16] - b[31:16];
      yi = a[15:0] - b[15:0];
      subc = {yr, yi};
    end
  endfunction

  function [31:0] mulc;
    input [31:0] a, b;
    logic [31:0] yr1, yr2, yi1, yi2;
    logic [15:0] ar, ai, br, bi, yyr1, yyr2, yyi1, yyi2, yr, yi;
    begin
        if( a[31] == 0 ) ar = a[31:16]; else ar = ~(a[31:16]-1);
        if( a[15] == 0 ) ai = a[15:0]; else ai = ~(a[15:0]-1);
        if( b[31] == 0 ) br = b[31:16]; else br = ~(b[31:16]-1);
        if( b[15] == 0 ) bi = b[15:0]; else bi = ~(b[15:0]-1);


        yr1 = ar * br;
        yr2 = ai * bi;

        yi1 = ar * bi;
        yi2 = ai * br;

        if( (a[31]^b[31])==0 ) yyr1 = yr1[26:11]; else yyr1 = ~yr1[26:11] + 1;
        if( (a[15]^b[15])==0 ) yyr2 = yr2[26:11]; else yyr2 = ~yr2[26:11] + 1;
        yr = yyr1 - yyr2;       

        if( (a[31]^b[15])==0 ) yyi1 = yi1[26:11]; else yyi1 = ~yi1[26:11] + 1;
        if( (a[15]^b[31])==0 ) yyi2 = yi2[26:11]; else yyi2 = ~yi2[26:11] + 1;
        yi = yyi1 + yyi2;       

      mulc = {yr, yi};
    end
  endfunction
endmodule