import React, {useCallback} from 'react';
import { Icon, Popover } from 'antd';
import styled from 'styled-components';
import Day from 'dayjs';

export interface Version {
  id: string,
  major_version: number,
  minor_version: number,
  build_number: number,
  description: string,
  branch_name: string,
  created_time: string,
}

interface ItemProps {
  version: Version,
  active?: boolean,
  onClick: (v: string) => void,
}

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
  max-width: 300px;
`;

const Description = styled.div`
  white-space: pre-wrap;
  word-break: break-all;
`;

const Item: React.FC<ItemProps> = props => {
  const { version, onClick } = props;
  const {
    major_version,
    minor_version,
    build_number,
    branch_name,
    description,
    created_time,
  } = version;
  const versionStr = `${major_version}.${minor_version}.${build_number}`;
  const clickCallback = useCallback(() => {
    onClick(versionStr);
  }, []);
  return (
    <Container onClick={clickCallback} active={props.active}>
      <Version>v{versionStr}</Version>
      <div>
        <Branch>{branch_name}</Branch>
        <Popover
          placement="bottomRight"
          arrowPointAtCenter
          content={(
            <ItemPopover>
              <Description>{description}</Description>
              <DateTime>{Day(created_time).format('YYYY-MM-DD HH:mm:ss')}</DateTime>
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
