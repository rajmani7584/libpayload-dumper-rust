use std::fs::File;
use std::io::Read;
use byte_unit::Byte;
use std::error::Error;

use crate::chromeos_update_engine;

use crate::chromeos_update_engine::DeltaArchiveManifest;
use crate::chromeos_update_engine::Signatures;

// struct Request {
//     partition: chromeos_update_engine::PartitionUpdate,  // Assuming proper binding for chromeos_update_engine
//     target_directory: String,
// }
struct PayloadHeader {
    version: u64,
    manifest_len: u64,
    metadata_signature_len: u32,
    size: u64
}

const PAYLOAD_HEADER_MAGIC: &str = "CrAU";
const BRILLO_MAJOR_PAYLOAD_VERSION: u64 = 2;
// const BLOCK_SIZE: u64 = 4096;


pub struct Payload {
    filename: String,
    file: Option<File>,  // Rust’s Option is used to handle None cases instead of nil
    header: Option<PayloadHeader>,
    delta_archive_manifest: Option<chromeos_update_engine::DeltaArchiveManifest>,  // Assuming protobuf is used
    signatures: Option<chromeos_update_engine::Signatures>,

    // concurrency: usize,
    metadata_size: i64,
    data_offset: i64,
    initialized: bool,

    // requests: Arc<Mutex<Vec<Request>>>,  // Using thread-safe types for concurrency
  }

impl Payload {
    pub fn new(filename: String) -> Payload {
        Payload {
            filename,
            file: None,
            header: None,
            delta_archive_manifest: None,
            signatures: None,
            // concurrency: 4,
            metadata_size: 0,
            data_offset: 0,
            initialized: false,
            // requests: Arc::new(Mutex::new(vec![]))
        }
    }

    pub fn init(&mut self) -> Result<String, Box<dyn Error>> {
        self.open()?;  // Open the payload file
        let mut msg: String = Default::default();

        let mut header = PayloadHeader {
            version: 0,
            manifest_len: 0,
            metadata_signature_len: 0,
            size: 0
        };


        header.read_from_payload(self)?;  // Read the header


        self.header = Some(header);
        // Read the manifest
        self.read_manifest()?;
        // Read metadata signature
        self.read_metadata_signature()?;

        self.metadata_size = (self.header.as_ref().unwrap().size + self.header.as_ref().unwrap().manifest_len) as i64;

        self.data_offset = self.metadata_size + self.header.as_ref().unwrap().metadata_signature_len as i64;



        if let Some(manifest) = &self.delta_archive_manifest {
           
            msg.insert_str(msg.len(), "Found partitions:");

            for (i, partition) in manifest.partitions.iter().enumerate() {
                let partition_name = &partition.partition_name;
                let partition_size = partition.new_partition_info.as_ref().map_or(0, |info| info.size.expect("info size not found"));

                let human_readable_size = Byte::from_u128(partition_size as u128)
                    .expect("Byte error")
                    .get_appropriate_unit(byte_unit::UnitType::Decimal)
                    .to_string();

                    let mg = format!("{}, ({})", partition_name, human_readable_size);
                msg.insert_str(msg.len(), mg.as_str());

                if i < manifest.partitions.len() - 1 {
                    print!(", ");
                } else {
                    println!();
                }
            }
        } else {
            println!("No partitions found.");
        }

        self.initialized = true;
        Ok(msg)
    }


    // fn read_data_blob(&self, offset: i64, length: i64) -> Result<Vec<u8>, Box<dyn Error>> {
    //     let mut buf = vec![0u8; length as usize];  // Create a buffer of the specified length
    //     let mut file = self.file.as_ref().ok_or("File is not opened")?; // Access the file from Payload

    //     // Move to the data offset and read from there
    //     file.seek(SeekFrom::Start((self.data_offset + offset) as u64))?;
    //     let n = file.read(&mut buf)?;

    //     // Ensure the number of bytes read matches the expected length
    //     if n as i64 != length {
    //         return Err(format!("Read length mismatch: {} != {}", n, length).into());
    //     }

    //     Ok(buf)  // Return the buffer if everything is successful
    // }

    // fn set_concurrency(&mut self, concurrency: usize) {
    //     self.concurrency = concurrency;
    // }

    // fn get_concurrency(&self) -> usize {
    //     self.concurrency
    // }

    fn open(&mut self) -> Result<(), Box<dyn Error>> {
        let file = File::open(&self.filename)?;
        self.file = Some(file);
        Ok(())
    }

    fn read_manifest(&mut self) -> Result<(), Box<dyn Error>> {
        let manifest_len = self.header.as_ref().unwrap().manifest_len as usize;
        let mut manifest_buf = vec![0; manifest_len];

        self.file.as_mut().unwrap().read_exact(&mut manifest_buf)?;

        let manifest: DeltaArchiveManifest = prost::Message::decode(&manifest_buf[..])?;
        self.delta_archive_manifest = Some(manifest);

        Ok(())
    }

    fn read_metadata_signature(&mut self) -> Result<(), Box<dyn Error>> {
        let metadata_signature_len = self.header.as_ref().unwrap().metadata_signature_len as usize;
        if metadata_signature_len == 0 {
            return Ok(());
        }

        let mut sig_buf = vec![0; metadata_signature_len];
        self.file.as_mut().unwrap().read_exact(&mut sig_buf)?;

        let signatures: Signatures = prost::Message::decode(&sig_buf[..])?;
        self.signatures = Some(signatures);

        Ok(())
    }

}


impl PayloadHeader {
    fn read_from_payload(&mut self, payload: &mut Payload) -> Result<(), Box<dyn Error>> {
        let mut buf = [0; 4];
        payload.file.as_mut().unwrap().read_exact(&mut buf)?;

        if std::str::from_utf8(&buf)? != PAYLOAD_HEADER_MAGIC {
            return Err("Invalid payload magic".into());
        }

        // Read Version
        let mut buf = [0; 8];
        payload.file.as_mut().unwrap().read_exact(&mut buf)?;
        self.version = u64::from_be_bytes(buf);
        println!("Payload Version: {}", self.version);

        if self.version != BRILLO_MAJOR_PAYLOAD_VERSION {
            return Err("Unsupported payload version".into());
        }

        // Read Manifest Len
        let mut buf = [0; 8];
        payload.file.as_mut().unwrap().read_exact(&mut buf)?;
        self.manifest_len = u64::from_be_bytes(buf);
        println!("Payload Manifest Length: {}", self.manifest_len);

        self.size = 24;

        // Read Manifest Signature Length
        let mut buf = [0; 4];
        payload.file.as_mut().unwrap().read_exact(&mut buf)?;
        self.metadata_signature_len = u32::from_be_bytes(buf);
        println!("Payload Manifest Signature Length: {}", self.metadata_signature_len);

        Ok(())
    }
}
