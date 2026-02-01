use crate::utils::file_utils::DelimitedIter;

trait FormatDebugger {
    fn report_panic(&mut self, iter:DelimitedIter);
    
}