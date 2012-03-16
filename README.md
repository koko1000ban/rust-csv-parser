CSV parser based on RFC 4180.

Example1
---------
 	csv::open("/path/to/csv_file") { |fields|
   		#info("%s",fields[0]);
	}
	
Example2
---------

	csv::parse("a,b,c") { |row|
   		let i = 0u;
   		while i < vec::len(row) {
      		io::println(#fmt("%u: %s", i, row[i]));
      		i += 1u;
   		}
	}
	
	# 0: a
	# 1: b
	# 2: c

