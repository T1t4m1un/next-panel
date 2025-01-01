import { FC } from 'react';
import './style.css';

export const Panel: FC = () => {
  return (
    <main className='panel'>
      <h1 className='panel-title'>Next Panel</h1>
      <div className='panel-search'>
        <input type='text' />
      </div>
      
    </main>
  );
};
