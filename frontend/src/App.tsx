import React from 'react';
import axios from 'axios';
import styled from 'styled-components';

export const Button = styled.div`
  height: 8px;
  width: 8px;
  border-radius: 50%;
  position: fixed;
  left: 8px;
  right: 8px;
  background-color: lightblue;
  cursor: pointer;
`;

export default () => {
  return <Button />
}
