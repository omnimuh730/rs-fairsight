import React from 'react';
import { Button } from '@mui/material';
import { useAnimations, animationStyles } from '../../utils/fantasyAnimations';

const FantasyButton = ({ 
  children, 
  colorScheme = 'primary',
  variant = 'contained',
  animation = 'scaleIn',
  delay = 0,
  sx = {},
  ...props 
}) => {
  useAnimations();

  const getAnimationStyle = () => {
    switch (animation) {
      case 'fadeInUp':
        return animationStyles.fadeInUp(delay);
      case 'scaleIn':
        return animationStyles.scaleIn(delay);
      case 'bounceIn':
        return animationStyles.bounceIn(delay);
      default:
        return {};
    }
  };

  const getColorGradient = () => {
    const gradients = {
      primary: 'linear-gradient(45deg, #2196F3, #21CBF3)',
      secondary: 'linear-gradient(45deg, #9C27B0, #E1BEE7)',
      success: 'linear-gradient(45deg, #4CAF50, #81C784)',
      warning: 'linear-gradient(45deg, #FF9800, #FFB74D)',
      info: 'linear-gradient(45deg, #00BCD4, #4DD0E1)',
    };
    return gradients[colorScheme] || gradients.primary;
  };

  return (
    <Button
      variant={variant}
      sx={{
        ...getAnimationStyle(),
        ...animationStyles.interactiveButton,
        background: variant === 'contained' ? getColorGradient() : 'transparent',
        borderRadius: '12px',
        textTransform: 'none',
        fontWeight: 600,
        padding: '10px 24px',
        boxShadow: variant === 'contained' ? '0 4px 15px rgba(0,0,0,0.2)' : 'none',
        border: variant === 'outlined' ? `2px solid ${getColorGradient()}` : 'none',
        color: variant === 'outlined' ? 'inherit' : 'white',
        position: 'relative',
        overflow: 'hidden',
        '&::before': {
          content: '""',
          position: 'absolute',
          top: 0,
          left: '-100%',
          width: '100%',
          height: '100%',
          background: 'linear-gradient(90deg, transparent, rgba(255,255,255,0.3), transparent)',
          transition: 'left 0.5s ease-in-out',
        },
        '&:hover::before': {
          left: '100%',
        },
        ...sx,
      }}
      {...props}
    >
      {children}
    </Button>
  );
};

export default FantasyButton;
