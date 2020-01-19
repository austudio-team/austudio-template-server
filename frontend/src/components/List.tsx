import React from 'react';
import Item from './Item';
import styled from 'styled-components';
import { Input, Icon } from 'antd';

const Title = styled.div`
  font-size: 10px;
  color: #999;
  margin: 8px 0 8px 16px;
  cursor: default;
`;

const StyledList = styled.div`
  max-height: 600px;
  min-width: 300px;
  overflow-y: hidden;
  display: flex;
  flex-direction: column;
`;

const ItemWrapper = styled.div`
  flex-grow: 1;
  overflow-y: auto;
`;

const HeaderWrapper = styled.div`
  display: flex;
  justify-content: space-between;
  align-items: center;
  flex-shrink: 0;
  border-bottom: 1px solid #eee;
`;
 
const List: React.FC = () => {
  return (
    <StyledList>
      <HeaderWrapper>
        <Title>Versions</Title>
        <Input
          style={{ width: 'auto', marginRight: 16 }}
          addonBefore={<Icon type="search" />}
          size="small"
          placeholder="Search"
        />
      </HeaderWrapper>
      <ItemWrapper>
        <Item active />
        <Item />
      </ItemWrapper>
    </StyledList>
  );
}

export default List;
