import React from 'react';
import styled from 'styled-components';
import List from './components/List';
import { Popover } from 'antd';
import './style.css';
import { PopoverProps } from 'antd/lib/popover';

const Button = styled.div`
  height: 8px;
  width: 8px;
  border-radius: 50%;
  position: fixed;
  left: 8px;
  top: 8px;
  background-color: lightblue;
  cursor: pointer;
`;

const WrappedPopover = (props: PopoverProps) => {
  return <Popover {...props} overlayClassName={props.className} />;
}

const NoPaddingPopover = styled(WrappedPopover)`
  .ant-popover-inner-content {
    overflow: auto;
    padding: 0;
    user-select: none;
  }
`;

export default () => {
  return (
    <NoPaddingPopover
      placement="topLeft"
      trigger="click"
      content={<List />}
    >
      <Button />
    </NoPaddingPopover>
  );
}
