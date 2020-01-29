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
  create_time: string,
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
  max-width: 200px;
`;

const Item: React.FC<ItemProps> = props => {
  const { version, onClick } = props;
  const {
    major_version,
    minor_version,
    build_number,
    branch_name,
    description,
    create_time,
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
          placement="bottom"
          content={(
            <ItemPopover>
              <div>{description}</div>
              <DateTime>{Day(create_time).format('YYYY-MM-DD HH:mm:ss')}</DateTime>
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
