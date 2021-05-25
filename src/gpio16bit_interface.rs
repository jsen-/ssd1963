pub trait WritePort {
    fn set_value(&mut self, value: u16);
    fn dir_write(&mut self);
}

pub trait ReadWritePort: WritePort {
    fn get_value(&mut self) -> u16;
    fn dir_write(&mut self);
    fn dir_read(&mut self);
}

#[must_use]
pub struct Committer<'a, WR, Error>
where
    WR: embedded_hal::digital::v2::OutputPin<Error = Error>,
{
    wr: &'a mut WR,
}

impl<'a, WR, Error> Committer<'a, WR, Error>
where
    WR: embedded_hal::digital::v2::OutputPin<Error = Error>,
{
    pub fn commit(&mut self) -> Result<(), Error> {
        self.wr.set_low()?;
        self.wr.set_high()?;
        Ok(())
    }
}

pub struct ValueSetter<'a, PortX, WR, Error>
where
    PortX: WritePort,
    WR: embedded_hal::digital::v2::OutputPin<Error = Error>,
{
    port: &'a mut PortX,
    wr: &'a mut WR,
}

impl<'a, PortX, WR, Error> ValueSetter<'a, PortX, WR, Error>
where
    PortX: WritePort,
    WR: embedded_hal::digital::v2::OutputPin<Error = Error>,
{
    pub fn set_value(&mut self, value: u16) -> Result<Committer<WR, Error>, Error> {
        self.port.set_value(value);
        Ok(Committer { wr: self.wr })
    }
}

pub struct Writer<'a, PortX, DC, WR, Error>
where
    PortX: WritePort,
    DC: embedded_hal::digital::v2::OutputPin<Error = Error>,
    WR: embedded_hal::digital::v2::OutputPin<Error = Error>,
{
    port: &'a mut PortX,
    dc: &'a mut DC,
    wr: &'a mut WR,
}

impl<'a, PortX, DC, WR, Error> Writer<'a, PortX, DC, WR, Error>
where
    PortX: WritePort,
    DC: embedded_hal::digital::v2::OutputPin<Error = Error>,
    WR: embedded_hal::digital::v2::OutputPin<Error = Error>,
{
    pub fn command(&mut self) -> Result<ValueSetter<PortX, WR, Error>, Error> {
        self.dc.set_low()?;
        Ok(ValueSetter {
            port: self.port,
            wr: self.wr,
        })
    }

    pub fn data(&mut self) -> Result<ValueSetter<PortX, WR, Error>, Error> {
        self.dc.set_high()?;
        Ok(ValueSetter {
            port: self.port,
            wr: self.wr,
        })
    }
}

pub struct ValueGetter<'a, PortX, RD, Error>
where
    PortX: ReadWritePort,
    RD: embedded_hal::digital::v2::OutputPin<Error = Error>,
{
    port: &'a mut PortX,
    rd: &'a mut RD,
}

impl<'a, PortX, RD, Error> ValueGetter<'a, PortX, RD, Error>
where
    PortX: ReadWritePort,
    RD: embedded_hal::digital::v2::OutputPin<Error = Error>,
{
    pub fn get_value(&mut self) -> Result<u16, Error> {
        let value = self.port.get_value();
        self.rd.set_high()?;
        Ok(value)
    }
}

pub struct Reader<'a, PortX, DC, RD, Error>
where
    PortX: ReadWritePort,
    DC: embedded_hal::digital::v2::OutputPin<Error = Error>,
    RD: embedded_hal::digital::v2::OutputPin<Error = Error>,
{
    port: &'a mut PortX,
    dc: &'a mut DC,
    rd: &'a mut RD,
}

impl<'a, PortX, DC, RD, Error> Reader<'a, PortX, DC, RD, Error>
where
    PortX: ReadWritePort,
    DC: embedded_hal::digital::v2::OutputPin<Error = Error>,
    RD: embedded_hal::digital::v2::OutputPin<Error = Error>,
{
    pub fn command(&mut self) -> Result<ValueGetter<PortX, RD, Error>, Error> {
        self.dc.set_low()?;
        Ok(ValueGetter {
            port: self.port,
            rd: self.rd,
        })
    }

    pub fn data(&mut self) -> Result<ValueGetter<PortX, RD, Error>, Error> {
        self.dc.set_high()?;
        Ok(ValueGetter {
            port: self.port,
            rd: self.rd,
        })
    }
}

pub trait WriteOnlyInterface {
    type Port: WritePort;
    type Error;
    type DC: embedded_hal::digital::v2::OutputPin<Error = Self::Error>;
    type WR: embedded_hal::digital::v2::OutputPin<Error = Self::Error>;
    fn write(&mut self) -> Result<Writer<Self::Port, Self::DC, Self::WR, Self::Error>, Self::Error>;
}

pub trait ReadWriteInterface: WriteOnlyInterface {
    type Port: ReadWritePort;
    type RD: embedded_hal::digital::v2::OutputPin<Error = Self::Error>;
    fn read(&mut self) -> Result<Reader<<Self as ReadWriteInterface>::Port, Self::DC, Self::RD, Self::Error>, Self::Error>;
}

pub struct GpioWriteOnly16BitInterface<Port, DC, WR> {
    port: Port,
    dc: DC,
    wr: WR,
}

impl<PortX, DC, WR, Error> GpioWriteOnly16BitInterface<PortX, DC, WR>
where
    PortX: WritePort,
    DC: embedded_hal::digital::v2::OutputPin<Error = Error>,
    WR: embedded_hal::digital::v2::OutputPin<Error = Error>,
{
    pub fn new(port: PortX, dc: DC, wr: WR) -> Self {
        Self { port, dc, wr }
    }

    pub fn release(self) -> (PortX, DC, WR) {
        (self.port, self.dc, self.wr)
    }
}

impl<PortX, DC, WR, Error> WriteOnlyInterface for GpioWriteOnly16BitInterface<PortX, DC, WR>
where
    PortX: WritePort,
    DC: embedded_hal::digital::v2::OutputPin<Error = Error>,
    WR: embedded_hal::digital::v2::OutputPin<Error = Error>,
{
    type Port = PortX;
    type Error = Error;
    type DC = DC;
    type WR = WR;
    fn write(&mut self) -> Result<Writer<PortX, DC, WR, Error>, Error> {
        self.wr.set_high()?;
        Ok(Writer {
            port: &mut self.port,
            dc: &mut self.dc,
            wr: &mut self.wr,
        })
    }
}

pub struct GpioReadWrite16BitInterface<Port, DC, WR, RD> {
    inner: GpioWriteOnly16BitInterface<Port, DC, WR>,
    rd: RD,
}

impl<PortX, DC, WR, RD, Error> GpioReadWrite16BitInterface<PortX, DC, WR, RD>
where
    PortX: ReadWritePort,
    DC: embedded_hal::digital::v2::OutputPin<Error = Error>,
    WR: embedded_hal::digital::v2::OutputPin<Error = Error>,
    RD: embedded_hal::digital::v2::OutputPin<Error = Error>,
{
    pub fn new(port: PortX, dc: DC, wr: WR, rd: RD) -> Self {
        Self {
            inner: GpioWriteOnly16BitInterface::new(port, dc, wr),
            rd,
        }
    }
}

impl<PortX, DC, WR, RD, Error> WriteOnlyInterface for GpioReadWrite16BitInterface<PortX, DC, WR, RD>
where
    PortX: WritePort,
    DC: embedded_hal::digital::v2::OutputPin<Error = Error>,
    WR: embedded_hal::digital::v2::OutputPin<Error = Error>,
    RD: embedded_hal::digital::v2::OutputPin<Error = Error>,
{
    type Port = PortX;
    type Error = Error;
    type DC = DC;
    type WR = WR;
    fn write(&mut self) -> Result<Writer<PortX, DC, WR, Error>, Error> {
        self.rd.set_high()?;
        self.inner.port.dir_write();
        self.inner.write()
    }
}

impl<PortX, DC, WR, RD, Error> ReadWriteInterface for GpioReadWrite16BitInterface<PortX, DC, WR, RD>
where
    PortX: ReadWritePort,
    DC: embedded_hal::digital::v2::OutputPin<Error = Error>,
    WR: embedded_hal::digital::v2::OutputPin<Error = Error>,
    RD: embedded_hal::digital::v2::OutputPin<Error = Error>,
{
    type Port = PortX;
    type RD = RD;
    fn read(&mut self) -> Result<Reader<PortX, DC, RD, Error>, Error> {
        self.inner.wr.set_high()?; // maybe not needed
        self.rd.set_low()?; // read
        self.inner.port.dir_read();

        Ok(Reader {
            port: &mut self.inner.port,
            dc: &mut self.inner.dc,
            rd: &mut self.rd,
        })
    }
}
