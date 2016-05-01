use std::io;
use super::Collector;
use DocId;
use SegmentReader;
use SegmentLocalId;

pub struct MultiCollector<'a> {
    collectors: Vec<&'a mut Collector>,
}

impl<'a> MultiCollector<'a> {
    pub fn from(collectors: Vec<&'a mut Collector>) -> MultiCollector {
        MultiCollector {
            collectors: collectors,
        }
    }
}

impl<'a> Collector for MultiCollector<'a> {

    fn set_segment(&mut self, segment_local_id: SegmentLocalId, segment: &SegmentReader) -> io::Result<()> {
        for collector in self.collectors.iter_mut() {
            try!(collector.set_segment(segment_local_id, segment));
        }
        Ok(())
    }

    fn collect(&mut self, doc_id: DocId) {
        for collector in self.collectors.iter_mut() {
            collector.collect(doc_id);
        }
    }
}