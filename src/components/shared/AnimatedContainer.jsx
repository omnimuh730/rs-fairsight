import React from 'react';
import { Box } from '@mui/material';
import { useAnimations, animationStyles } from '../../utils/fantasyAnimations';

const AnimatedContainer = ({ 
  children, 
  animation = 'fadeInUp', 
  delay = 0, 
  sx = {},
  ...props 
}) => {
  useAnimations();
  
  const getAnimationStyle = () => {
    switch (animation) {
      case 'fadeInUp':
        return animationStyles.fadeInUp(delay);
      case 'fadeInDown':
        return animationStyles.fadeInDown(delay);
      case 'slideInLeft':
        return animationStyles.slideInLeft(delay);
      case 'slideInRight':
        return animationStyles.slideInRight(delay);
      case 'scaleIn':
        return animationStyles.scaleIn(delay);
      case 'rotateIn':
        return animationStyles.rotateIn(delay);
      case 'bounceIn':
        return animationStyles.bounceIn(delay);
      default:
        return {};
    }
  };

  return (
    <Box
      sx={{
        ...getAnimationStyle(),
        ...sx,
      }}
      {...props}
    >
      {children}
    </Box>
  );
};

export default AnimatedContainer;
