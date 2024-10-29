use std::cell::OnceCell;


fn main(){
    let args:Vec<String>  = std::env::args().collect();
    let wat_form = std::fs::read_to_string(&args[1]).unwrap();
    let tokens_of_wat = wat_form.split('\n');
    let mut instrumented_wat_lines:Vec<String> = vec![];
    let mut func_index = 0;
    for line in tokens_of_wat{
        instrumented_wat_lines.push(line.to_string());
        let section_block = line.trim();
        if section_block.starts_with("(func "){
            let mut match_bracket = 0;
            for c in line.chars(){
                if c == '('{
                    match_bracket += 1;
                }
                else if c == ')'{
                    match_bracket -=1;
                }
            }
            func_index += 1;
            
            for sub_block_in_func_section in section_block.split('('){
                if sub_block_in_func_section.starts_with(";"){
                    if match_bracket == 1{
                        instrumented_wat_lines.push(format!("    i32.const {}", func_index));
                        instrumented_wat_lines.push(        "    call $inc_call_cnt".to_string());
                    }else if match_bracket == 0{
                        instrumented_wat_lines.pop();
                        instrumented_wat_lines.push(line[..line.len() - 1].to_string());
                        instrumented_wat_lines.push(format!("    i32.const {}", func_index));
                        instrumented_wat_lines.push(        "    call $inc_call_cnt".to_string());
                        instrumented_wat_lines.push(        "  )".to_string());
                    }else{
                        unreachable!()
                    }
                    break;
                }
            }
            
        }else if section_block.starts_with("(table"){
            let origin_table_info = instrumented_wat_lines.pop().unwrap();
            instrumented_wat_lines.push(format!("  (import \"instrumentation\" \"increase_call_count\" (func $inc_call_cnt (type 0)))"));
            instrumented_wat_lines.push(origin_table_info);
        }else if section_block.starts_with("(local "){
            let origin_local_var_info = instrumented_wat_lines.pop().unwrap();
            let inst2 = instrumented_wat_lines.pop().unwrap();
            let inst1 = instrumented_wat_lines.pop().unwrap();
            instrumented_wat_lines.push(origin_local_var_info);
            instrumented_wat_lines.push(inst1);
            instrumented_wat_lines.push(inst2);
        }else if section_block.starts_with("(import"){
            func_index += 1;
        }

    }   
   std::fs::write(&args[2], instrumented_wat_lines.join("\n")).unwrap();
}
