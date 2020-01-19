import React from 'react';
import { Icon, Popover } from 'antd';
import styled from 'styled-components';

const Container = styled.div<{ active?: boolean }>`
  padding: 6px 16px;
  cursor: pointer;
  display: flex;
  justify-content: space-between;
  color: ${({ active }) => active ? '#1890ff' : 'inherit'};

  :hover {
    background-color: #eee;
  }

  :active {
    background-color: #ddd;
  }
`;

const Version = styled.div`
  font-weight: 500;
`;

const Branch = styled.span`
  margin-left: 12px;
  margin-right: 6px;
`;

const DateTime = styled.div`
  font-size: 10px;
  color: #999;
  margin-top: 8px;
`;

const ItemPopover = styled.div`
  user-select: none;
  cursor: default;
  max-width: 200px;
`;

interface ItemProps {
  active?: boolean;
};

const Item: React.FC<ItemProps> = props => {
  return (
    <Container active={props.active}>
      <Version>v1.0.0</Version>
      <div>
        <Branch>feat-create-audio</Branch>
        <Popover
          placement="bottom"
          content={(
            <ItemPopover>
              <div>feat: add audio create entry</div>
              <DateTime>2020/01/02 11:11:21</DateTime>
            </ItemPopover>
          )}
        >
          <Icon type="info-circle" />
        </Popover>
      </div>
    </Container>
  );
}

export default Item;
