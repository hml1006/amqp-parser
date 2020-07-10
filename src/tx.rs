use amqp_types::frame::{Arguments, TxSelect, TxSelectOk, TxCommit, TxCommitOk, TxRollback, TxRollbackOk};
use crate::error::FrameDecodeErr;
use nom::number::complete::be_u8;
use nom::error::ErrorKind;

pub(crate) fn parse_tx_select(buffer: &[u8]) -> Result<Arguments, FrameDecodeErr> {
    let (_, dummy) = match be_u8::<(_, ErrorKind)>(buffer) {
        Ok(ret) => ret,
        Err(e) => return Err(FrameDecodeErr::ParseFrameFailed)
    };
    let tx_select = TxSelect::default();
    tx_select.set_dummy(dummy);
    Ok(Arguments::TxSelect(tx_select))
}

pub(crate) fn parse_tx_select_ok(buffer: &[u8]) -> Result<Arguments, FrameDecodeErr> {
    let (_, dummy) = match be_u8::<(_, ErrorKind)>(buffer) {
        Ok(ret) => ret,
        Err(e) => return Err(FrameDecodeErr::ParseFrameFailed)
    };
    let tx_select_ok = TxSelectOk::default();
    tx_select_ok.set_dummy(dummy);
    Ok(Arguments::TxSelectOk(tx_select_ok))
}

pub(crate) fn parse_tx_commit(buffer: &[u8]) -> Result<Arguments, FrameDecodeErr> {
    let (_, dummy) = match be_u8::<(_, ErrorKind)>(buffer) {
        Ok(ret) => ret,
        Err(e) => return Err(FrameDecodeErr::ParseFrameFailed)
    };
    let tx_commit = TxCommit::default();
    tx_commit.set_dummy(dummy);
    Ok(Arguments::TxCommit(tx_commit))
}

pub(crate) fn parse_tx_commit_ok(buffer: &[u8]) -> Result<Arguments, FrameDecodeErr> {
    let (_, dummy) = match be_u8::<(_, ErrorKind)>(buffer) {
        Ok(ret) => ret,
        Err(e) => return Err(FrameDecodeErr::ParseFrameFailed)
    };
    let tx_commit_ok = TxCommitOk::default();
    tx_commit_ok.set_dummy(dummy);
    Ok(Arguments::TxCommitOk(tx_commit_ok))
}

pub(crate) fn parse_tx_rollback(buffer: &[u8]) -> Result<Arguments, FrameDecodeErr> {
    let (_, dummy) = match be_u8::<(_, ErrorKind)>(buffer) {
        Ok(ret) => ret,
        Err(e) => return Err(FrameDecodeErr::ParseFrameFailed)
    };
    let tx_rollback = TxRollback::default();
    tx_rollback.set_dummy(dummy);
    Ok(Arguments::TxRollback(tx_rollback))
}

pub(crate) fn parse_tx_rollback_ok(buffer: &[u8]) -> Result<Arguments, FrameDecodeErr> {
    let (_, dummy) = match be_u8::<(_, ErrorKind)>(buffer) {
        Ok(ret) => ret,
        Err(e) => return Err(FrameDecodeErr::ParseFrameFailed)
    };
    let tx_rollback_ok = TxRollbackOk::default();
    tx_rollback_ok.set_dummy(dummy);
    Ok(Arguments::TxRollbackOk(tx_rollback_ok))
}
