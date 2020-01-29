import React, {useCallback, useEffect, useRef, useState} from 'react';
import Item, {Version} from './Item';
import styled from 'styled-components';
import {Input, Icon, Button, Spin, Empty} from 'antd';
import axios from 'axios';
import Cookie from 'js-cookie';
import debounce from 'lodash.debounce';

const Title = styled.div`
  font-size: 10px;
  color: #999;
  margin: 8px 0 8px 16px;
  cursor: default;
`;

const StyledList = styled.div`
  min-width: 300px;
  overflow-y: hidden;
  display: flex;
  flex-direction: column;
`;

const ItemWrapper = styled.div`
  max-height: 500px;
  min-height: 180px;
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

interface VersionResponse {
  versions: Version[],
  versionsCount: number,
}

const useFetch = () => {
  const [items, setItems] = useState<Version[]>([]);
  const [isLastPage, setIsLastPage] = useState<boolean>(false);
  const [fetching, setFetching] = useState<boolean>(false);
  const offset = useRef<number>(0);
  const limit = useRef<number>(20);
  const getList = useCallback(() => {
    if (fetching || isLastPage) return;
    setFetching(true);
    axios.get<VersionResponse>('/template/api/versions', {
      params: {
        offset: offset.current,
        limit: limit.current,
      },
    }).then(v => {
      const { data } = v;
      if (data.versionsCount < limit.current) {
        setIsLastPage(true);
      }
      offset.current += limit.current;
      setItems(items.concat(data.versions));
      setFetching(false);
    }).catch(() => {
      setFetching(false);
    });
  }, [fetching, isLastPage, items]);

  return [items, isLastPage, fetching, getList];
}
 
const List: React.FC = () => {
  const [items, isLastPage, fetching, getList] = useFetch();
  const [currentVersion, setCurrentVersion] = useState<string | null>(null);
  const scroller = useRef<HTMLDivElement>();
  useEffect(() => {
    getList();
    const cookie = Cookie.get('version');
    cookie && setCurrentVersion(cookie);
  }, []);
  const handleItemClick = useCallback((v: string) => {
    Cookie.set('version', v, { expires: 7 });
    window.location.reload();
  }, []);
  const handleFollowClick = useCallback(() => {
    Cookie.remove('version');
    window.location.reload();
  }, []);
  const handleScroll = useCallback(debounce((e) => {
    const current = scroller.current;
    if (fetching || isLastPage || !current) return;
    const {clientHeight, scrollHeight, scrollTop} = current;
    if (scrollTop + clientHeight > scrollHeight - 100) {
      getList();
    }
  }, 100), [isLastPage, getList, fetching]);
  return (
    <StyledList>
      <HeaderWrapper>
        <Title>Versions</Title>
        {/*<Input*/}
        {/*  style={{ width: 'auto', marginRight: 16 }}*/}
        {/*  addonBefore={<Icon type="search" />}*/}
        {/*  size="small"*/}
        {/*  placeholder="Search"*/}
        {/*/>*/}
        <Button onClick={handleFollowClick} size="small" style={{ marginRight: 16 }}>Follow Latest Version</Button>
      </HeaderWrapper>
      <Spin spinning={fetching}>
        <ItemWrapper onScroll={handleScroll} ref={scroller}>
          {
            items.map((v, i) => {
              const version = `${v.major_version}.${v.minor_version}.${v.build_number}`;
              const active = (!currentVersion && i === 0) || currentVersion === version;
              return <Item key={version} version={v} onClick={handleItemClick} active={active} />;
            })
          }
          {
            items.length === 0 && <Empty />
          }
        </ItemWrapper>
      </Spin>
    </StyledList>
  );
}

export default List;
